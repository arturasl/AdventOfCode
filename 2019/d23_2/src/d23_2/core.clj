(ns d23-2.core
  (:gen-class)
  (:require [intcode.core :as code]))

(defn init-network [s size]
  (let [program (->> s code/str->memory code/init-program)]
    (vec (map #(assoc (code/exec (code/to->stdin program [%]))
                      :network-idx %)
              (range size)))))

(defn to->stdin-of-network [state idx in]
  (if (= idx 255) (assoc state :nat in)
      (let [_ (assert (< idx (:num-machines state)))
            state (update-in state [:network idx] #(code/exec (code/to->stdin % in)))
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
