(ns d23-1.core
  (:gen-class)
  (:require [intcode.core :as code]))

(defn init-network [s size]
  (let [program (->> s code/str->memory code/init-program)]
    (vec (map #(assoc (code/exec (code/to->stdin program [%]))
                      :network-idx %)
              (range size)))))

(defn print-network [network]
  (doseq [[idx machine] (map-indexed vector network)]
    (println "idx:" idx
             "pointer" (:pointer machine)
             "state:" (:state machine)
             "output:" (:output machine))))

(defn find-first-with-output [network]
  (some #(and (not-empty (:output %))
              %)
        network))

(defn to->stdin-of-network [network idx in]
  (update network idx #(code/exec (code/to->stdin % in))))

(defn read-msg-from-network [network read-idx]
  (let [machine (get network read-idx)
        output (:output machine)
        _ (assert (not-empty output) machine)
        _ (assert (zero? (mod (count output) 3)))
        [idx x y] output]
    [(update network read-idx (fn [machine] (update machine :output (partial drop 3))))
     [idx x y]]))

(defn create-packet [network]
  (let [first-with-output (find-first-with-output network)]
    (if first-with-output
      (let [[network [idx x y]] (read-msg-from-network network (:network-idx first-with-output))]
        {:network network :to idx :msg [x y]})
      (let [attempt-idx (rand-int (count network))]
        {:network network :to attempt-idx :msg [-1]}))))

(defn solve [s]
  ; (let [network (init-network s 50)
  ;       network (to->stdin-of-network network 0 [-1])
  ;       _ (print-network network)
  ;       [network [idx x y]] (read-msg-from-network network 0)]
  ;   (println idx x y)
  ;   (print-network network)))
  (loop [network (init-network s 50)
         its 0
         msgs-sent 0]
    (let [{:keys [network to msg]} (create-packet network)]
      (cond
        (= to 255) (println msg)
        (> its 10000) (print-network network)
        :else (do
                (println "to:" to "msg:" msg "sent:" msgs-sent)
                (recur (to->stdin-of-network network to msg)
                       (inc its)
                       (if (not= (first msg) -1) (inc msgs-sent) msgs-sent)))))))

(defn -main
  [& _]
  (solve (slurp *in*)))
