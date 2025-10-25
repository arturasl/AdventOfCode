(ns d02-1.core
  (:gen-class)
  (:require [clojure.string :as str]))

(defn exec-bin [program pos op]
  (let [[_ a-addr b-addr dest-addr] (subvec program pos (+ pos 4))]
    (assoc program
           dest-addr
           (op (get program a-addr)
               (get program b-addr)))))

(subvec [10 11 12 13 14 15] 0 (+ 0 4))

(defn exec
  ([program] (exec program 0))
  ([program pos]
   (case (get program pos)
     1 (recur (exec-bin program pos +) (+ pos 4))
     2 (recur (exec-bin program pos *) (+ pos 4))
     99 program
     (if (>= pos (count program))
       (throw (RuntimeException. "Out of bounds"))
       (throw (RuntimeException. (str "Unknown op code: " (get program pos))))))))

(defn str->program [s]
  (->> (str/split s #",")
       (map str/trim)
       (remove empty?)
       (map parse-long)
       (into [])))

(defn solve [s]
  (-> (str->program s)
      (assoc 1 12)
      (assoc 2 2)
      exec
      (get 0)))

(defn -main
  [& _]
  (println (solve (slurp "large.in"))))
