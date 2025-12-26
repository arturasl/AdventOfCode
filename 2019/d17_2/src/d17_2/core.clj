(ns d17-2.core
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
          (throw (ex-info "Unknown op in instruction"
                          {:program program :instruction instruction}))))))

(defn output-to-grid [output]
  (let [cells (->> output
                   (map char)
                   (apply str)
                   (str/split-lines)
                   (map vec)
                   vec)
        height (count cells)
        width (count (first cells))]
    {:height height
     :width width
     :cells cells}))

(defn read-grid [memory]
  (->> memory
       init-program
       exec
       :output
       output-to-grid))

(defn print-grid [{:keys [cells]}]
  (println (str/join "\n" (map str/join cells))))

(defn ins-to-bytes [ins]
  (let [ins-str-arr (map (fn [val]
                           (if (keyword? val)
                             (name val)
                             (str val)))
                         ins)]
    (conj (vec (map int (str/join "," ins-str-arr)))
          (int \newline))))

(deftest test-ins-to-bytes
  (is (= [65 44 66 44 49 48 44 76 10]
         (ins-to-bytes [:A :B 10 "L"])))
  (is (= [65 44 66 44 67 44 66 44 65 44 67 10]
         (ins-to-bytes [:A :B :C :B :A :C])))
  (is (= [82 44 56 44 82 44 56 10]
         (ins-to-bytes ["R" 8 "R" 8])))
  (is (= [82 44 52 44 82 44 52 44 82 44 56 10]
         (ins-to-bytes ["R" 4 "R" 4 "R" 8])))
  (is (= [76 44 54 44 76 44 50 10]
         (ins-to-bytes ["L" 6 "L" 2]))))

; L 4 L 4 L 6 R 10 L 6 L 4 L 4 L 6 R 10 L 6 L 12 L 6 R 10 L 6 R 8
; R 10 L 6 R 8 R 10 L 6 L 4 L 4 L 6 R 10 L 6 R 8 R 10 L 6 L 12 L 6
; R 10 L 6 R 8 R 10 L 6 L 12 L 6 R 10 L 6

; A A B C C A C B C B
; A: L 4 L 4 L 6 R 10 L 6
; B: L 12 L 6 R 10 L 6
; C: R 8 R 10 L 6

(defn solve [s]
  (let [memory (str->memory s)
        grid (read-grid memory)
        _ (print-grid grid)
        memory (assoc memory 0 2)
        program (exec (to->stdin (init-program memory)
                                 (concat (ins-to-bytes [:A :A :B :C :C :A :C :B :C :B])
                                         (ins-to-bytes ["L" 4 "L" 4 "L" 6 "R" 10 "L" 6])
                                         (ins-to-bytes ["L" 12 "L" 6 "R" 10 "L" 6])
                                         (ins-to-bytes ["R" 8 "R" 10 "L" 6])
                                         [(int \n) (int \newline)])))]
    (println (:output program))))

(defn -main
  [& _]
  (solve (slurp  "./large.in")))
