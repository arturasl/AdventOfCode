(ns d15-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.data.finger-tree :as finger]))

(defn str->memory [s]
  (->> (str/split s #",")
       (map str/trim)
       (remove empty?)
       (map parse-long)
       (into [])))

(defn memory->map [memory]
  (if (map? memory)
    memory
    (zipmap (range) memory)))

(defn init-program [base]
  (if (map? base)
    (let [merged (merge {:memory {0 99}
                         :input []
                         :output []
                         :pointer 0
                         :state
                         :ready
                         :relative-base 0}
                        base)]
      (assoc merged :memory (memory->map (:memory merged))))
    (init-program {:memory base})))

(defn to->stdin [program vals]
  (let [program (update program :input into vals)]
    (if (and (not-empty vals) (= (:state program) :waiting-read))
      (assoc program :state :ready)
      program)))

(defn opcode->op-data [opcode]
  (case (mod opcode 100)
    1 {:op :sum :num-params 3}
    2 {:op :mul :num-params 3}
    3 {:op :read :num-params 1}
    4 {:op :write :num-params 1}
    5 {:op :jump-if-true :num-params 2}
    6 {:op :jump-if-false :num-params 2}
    7 {:op :less-than :num-params 3}
    8 {:op :equal :num-params 3}
    9 {:op :adjust-relative-base :num-params 1}
    99 {:op :halt :num-params 0}
    (throw (ex-info "Unknown opcode" {:opcode opcode}))))

(defn parse-addressing [mode]
  (case mode
    \0 :position
    \1 :immediate
    \2 :relative
    (throw (ex-info "Unknown addressing:" {:mode mode}))))

(defn get-memory
  ([program] (get-memory program (:pointer program)))
  ([program pos] (get (:memory program) pos 0)))

(defn put-memory [program pos val]
  (assoc-in program [:memory pos] val))

(defn parse-instruction [program]
  (let [opcode (get-memory program)
        {:keys [op num-params]} (opcode->op-data opcode)
        str-addressings (reverse (if (>= opcode 100) (str (quot opcode 100)) ""))
        str-addressings-full (->> (repeat (- num-params (count str-addressings)) \0)
                                  (concat str-addressings)
                                  (apply str))
        addressings (vec (map parse-addressing str-addressings-full))]
    {:op op
     :addressings addressings
     :pointer (:pointer program)
     :num-params num-params}))

(defn resolve-raw-param [program instruction pos]
  (get-memory program (+ (:pointer instruction) pos 1)))

(defn resolve-param [program instruction pos]
  (let [mode (get (:addressings instruction) pos)
        raw-param (resolve-raw-param program instruction pos)]
    (case mode
      :immediate raw-param
      :position (get-memory program raw-param)
      :relative (get-memory program (+ (:relative-base program) raw-param))
      (throw (ex-info "Unknown mode" mode)))))

(defn resolve-addr [program instruction pos]
  (let [mode (get (:addressings instruction) pos)
        raw-param (resolve-raw-param program instruction pos)]
    (case mode
      :immediate raw-param
      :position raw-param
      :relative (+ (:relative-base program) raw-param)
      (throw (ex-info "Unknown mode" mode)))))

(defn exec-bin-mem [program instruction f]
  (let [a (resolve-param program instruction 0)
        b (resolve-param program instruction 1)
        result-addr (resolve-addr program instruction 2)]
    (-> program
        (put-memory result-addr (f a b))
        (update :pointer #(+ 4 %)))))

(defn exec-read [program instruction]
  (let [result-addr (resolve-addr program instruction 0)
        input (:input program)]
    (if (empty? input)
      (assoc-in program [:state] :waiting-read)
      (-> program
          (update :input rest)
          (update :pointer #(+ 2 %))
          (put-memory result-addr (first input))))))

(defn exec-write [program instruction]
  (let [val (resolve-param program instruction 0)]
    (-> program
        (update :output #(conj % val))
        (update :pointer #(+ 2 %)))))

(defn exec-cond-jump [program instruction f]
  (let [a (resolve-param program instruction 0)
        b (resolve-param program instruction 1)]
    (if (f a)
      (assoc program :pointer b)
      (update program :pointer #(+ 3 %)))))

(defn exec-adjust-relative-base [program instruction]
  (let [a (resolve-param program instruction 0)]
    (-> program
        (update :relative-base #(+ % a))
        (update :pointer #(+ 2 %)))))

(defn exec-halt [program]
  (assoc program :state :halt))

(defn exec [program]
  (let [state (:state program)
        instruction (parse-instruction program)]
    (if (not= state :ready) program
        (case (:op instruction)
          :sum (recur (exec-bin-mem program instruction +))
          :mul (recur (exec-bin-mem program instruction *))
          :read (recur (exec-read program instruction))
          :write (recur (exec-write program instruction))
          :jump-if-true (recur (exec-cond-jump program instruction #(not= 0 %)))
          :jump-if-false (recur (exec-cond-jump program instruction #(= 0 %)))
          :less-than (recur (exec-bin-mem program instruction #(if (< % %2) 1 0)))
          :equal (recur (exec-bin-mem program instruction #(if (= % %2) 1 0)))
          :adjust-relative-base (recur (exec-adjust-relative-base program instruction))
          :halt (exec-halt program)
          (throw (ex-info "Unknown op in instruction" {:program program :instruction instruction}))))))

(defn grid->str [grid robot]
  (let [ys (filter (comp not nil?) (conj (map :y (keys grid)) (:y robot)))
        xs (filter (comp not nil?) (conj (map :x (keys grid)) (:x robot)))
        [min-y max-y] (map #(apply % ys) [min max])
        [min-x max-x] (map #(apply % xs) [min max])
        height (inc (- max-y min-y))
        width (inc (- max-x min-x))
        drawn (reduce (fn [drawn-so-far [y x]]
                        (assoc-in drawn-so-far [y x]
                                  (let [pos {:x (+ x min-x) :y (+ y min-y)}]
                                    (if (= pos robot) "r"
                                        (case (get grid pos :unknown)
                                          :wall "#"
                                          :unknown "?"
                                          :oxygen "_"
                                          :empty " ")))))
                      (into [] (repeat height (into [] (repeat width " "))))
                      (for [y (range height) x (range width)] [y x]))]
    (str/join "\n" (map str/join drawn))))

(defn add [pos1 pos2]
  (let [coords [:y :x]]
    (zipmap coords
            (map #(+ (% pos1) (% pos2)) coords))))

(def moves [{:id 1 :rel-pos {:x 0 :y -1}},
            {:id 2 :rel-pos {:x 0 :y 1}},
            {:id 3 :rel-pos {:x -1 :y 0}},
            {:id 4 :rel-pos {:x 1 :y 0}}])

(defn find-closest [grid start search-for]
  (loop [space (finger/double-list {:poses [start] :path []})
         visited #{start}
         max-poses 1]
    (if (empty? space)
      (do
        (println max-poses)
        nil)
      (let [{:keys [poses path] :as state} (first space)
            pos (last poses)
            space (rest space)
            around (map #(assoc % :abs-pos (add (:rel-pos %) pos)) moves)
            valid-around (remove (fn [{:keys [abs-pos]}]
                                   (or (contains? visited abs-pos)
                                       (= (get grid abs-pos) :wall)))
                                 around)
            next-visited (into visited (map :abs-pos valid-around))
            next-space (apply conj space (map #(hash-map
                                                :poses (conj poses (:abs-pos %))
                                                :path (conj path (:id %)))
                                              valid-around))]
        (if (= (get grid pos :unknown) search-for)
          state
          (recur next-space next-visited (max max-poses (count (:poses (last next-space))))))))))

(defn resolve-grid [program]
  (loop [program program
         robot {:x 0 :y 0}
         grid {robot :empty}]
    (println)
    (println (grid->str grid robot))
    (let [found-closest (find-closest grid robot :unknown)]
      (if (nil? found-closest)
        grid
        (let [ids-till-unknown (:path found-closest)
              poses-till-unknown (:poses found-closest)
              next-program (-> program (to->stdin (reverse ids-till-unknown)) exec)
              _ (assert (every? #(= % 1) (butlast (:output next-program)))
                        (str (:output next-program) robot poses-till-unknown ids-till-unknown))
              out (last (:output next-program))
              next-grid (assoc grid (last poses-till-unknown)
                               (case out
                                 0 :wall
                                 1 :empty
                                 2 :oxygen))
              next-robot (if (zero? out)
                           (last (butlast poses-till-unknown))
                           (last poses-till-unknown))]
          (recur (assoc next-program :output []) next-robot next-grid))))))

(defn solve [s]
  (let [grid
        (-> s
            str->memory
            init-program
            exec
            resolve-grid)
        found-oxygen (find-closest grid {:x 0 :y 0} :oxygen)]
    (find-closest grid (last (:poses found-oxygen)) :does-not-exist)))

; 285 -- too high

(defn -main
  [& _]
  (println (solve (slurp  "./large.in"))))
