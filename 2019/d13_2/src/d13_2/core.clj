(ns d13-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]))

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

(defn parse-tile-id [id]
  (case id
    0 :empty
    1 :wall
    2 :block
    3 :horizontal-paddle
    4 :ball
    (throw (ex-info (str "Unknown id: " id) {:id id}))))

(defn parse-tile-instruction [[x y id]]
  (if (= [y x] [0 -1])
    [:score id]
    [{:y y :x x} (parse-tile-id id)]))

(defn parse-tile-instructions [output]
  (->> output
       (partition 3)
       (map parse-tile-instruction)))

(deftest test-parse-tile-instructions
  (is (= [[{:y 2 :x 1} :horizontal-paddle] [{:y 5 :x 6} :ball]]
         (parse-tile-instructions [1,2,3,6,5,4]))))

(defn grid->str [grid]
  (let [ys (filter (comp not nil?) (map :y (keys grid)))
        xs (filter (comp not nil?) (map :x (keys grid)))
        [min-y max-y] (map #(apply % ys) [min max])
        [min-x max-x] (map #(apply % xs) [min max])
        height (inc (- max-y min-y))
        width (inc (- max-x min-x))
        drawn (reduce (fn [drawn-so-far [y x]]
                        (assoc-in drawn-so-far [y x]
                                  (case (get grid {:y (+ y min-y) :x (+ x min-x)} :empty)
                                    :empty " "
                                    :wall "█"
                                    :block "■"
                                    :horizontal-paddle "―"
                                    :ball "⬤")))
                      (into [] (repeat height (into [] (repeat width " "))))
                      (for [y (range height) x (range width)] [y x]))]
    (str (str/join "\n" (map str/join drawn)) "\nScore: " (:score grid))))

(defn program->grid [program]
  (->> (:output program)
       (parse-tile-instructions)
       flatten
       (apply hash-map)))

(defn get-item-x [grid item]
  (some #(and (= (second %) item) (:x (first %))) grid))

(defn get-next-move [grid]
  (compare (get-item-x grid :ball) (get-item-x grid :horizontal-paddle)))

(defn solve [s]
  (loop [program (-> s str->memory init-program (assoc-in [:memory 0] 2) exec)
         grid (program->grid program)]
    (println (grid->str grid))
    (if (= (:state program) :halt)
      program
      (do
        (assert (= (:state program) :waiting-read))
        (let [program-wo-output (assoc program :output [])
              next-program (exec (to->stdin program-wo-output [(get-next-move grid)]))
              next-grid (merge grid (program->grid next-program))]
          (recur next-program next-grid))))))

(defn -main
  [& _]
  (solve (slurp  "./large.in")))
