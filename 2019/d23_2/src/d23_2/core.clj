(ns d23-2.core
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

(type (drop 3 (finger/double-list 1 2 3 4 5)))

(defn init-program [base]
  (if (map? base)
    (let [merged (merge {:memory {0 99}
                         :input (finger/double-list)
                         :output (finger/double-list)
                         :pointer 0
                         :state :ready
                         :relative-base 0}
                        base)]
      (assoc merged :memory (memory->map (:memory merged))))
    (init-program {:memory base})))

(defn to->stdin [program vals]
  (assert (vector? vals) (str "Expected a vector, but got:" vals))
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
        input (:input program)
        _ (assert (instance? clojure.data.finger_tree.DoubleList input))]
    (if (empty? input)
      (assoc-in program [:state] :waiting-read)
      (-> program
          (update :input rest)
          (update :pointer #(+ 2 %))
          (put-memory result-addr (first input))))))

(defn exec-write [program instruction]
  (let [val (resolve-param program instruction 0)
        _ (assert (instance? clojure.data.finger_tree.DoubleList (:output program)))]
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

(defn init-network [s size]
  (let [program (->> s str->memory init-program)]
    (vec (map #(assoc (exec (to->stdin program [%]))
                      :network-idx %)
              (range size)))))

(defn to->stdin-of-network [state idx in]
  (if (= idx 255) (assoc state :nat in)
      (let [_ (assert (< idx (:num-machines state)))
            state (update-in state [:network idx] #(exec (to->stdin % in)))
            has-output (not-empty (get-in state [:network idx :output]))]
        (-> state
            (update :with-output (if has-output #(conj % idx) identity))
          ; Just tried it so no need to try until output is consumed.
            (update :to-try disj idx)))))

(defn read-msg-from-network [state read-idx]
  (let [_ (assert (< read-idx (:num-machines state)))
        _ (assert (contains? (:with-output state) read-idx))
        machine (get-in state [:network read-idx])
        output (:output machine)
        _ (assert (not-empty output) machine)
        _ (assert (zero? (mod (count output) 3)))
        [idx x y] output
        state (update-in state [:network read-idx]
                         (fn [machine] (update machine :output (comp rest rest rest))))
        state (if (= (count output) 3)
                (-> state
                    (update-in [:with-output] disj read-idx)
                    (update-in [:to-try] conj read-idx))
                state)]
    [state [idx x y]]))

(defn create-packet [state]
  (cond
    (not-empty (:with-output state))
    (let [with-output-idx (first (:with-output state))
          [state [idx x y]] (read-msg-from-network state with-output-idx)]
      {:state state :to idx :msg [x y]})
    ;
    (not-empty (:to-try state))
    (let [attempt-idx (first (:to-try state))]
      {:state state :to attempt-idx :msg [-1]})
    ;
    :else
    {:state state :to 0 :msg (:nat state) :is-nat-send true}))

(defn solve [s num-machines]
  (loop [state {:num-machines num-machines
                :network (init-network s num-machines)
                :to-try (into #{} (range num-machines))
                :with-output #{}
                :nat nil
                :prev-nat-send nil}
         its 0]
    (let [{:keys [state to msg is-nat-send]} (create-packet state)]
      (if (and is-nat-send
               (= (:prev-nat-send state) msg)) (println msg)
          (recur (-> state
                     (assoc :prev-nat-send (if is-nat-send msg identity))
                     (to->stdin-of-network to msg))
                 (inc its))))))

(defn -main
  [& _]
  (solve (slurp *in*) 50))
