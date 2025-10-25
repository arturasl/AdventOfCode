(ns d02-2.core
  (:gen-class)
  (:require [clojure.string :as str]))

(defn exec-bin [program pos op]
  (let [[_ a-addr b-addr dest-addr] (subvec program pos (+ pos 4))]
    (assoc program
           dest-addr
           (op (get program a-addr)
               (get program b-addr)))))

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

(defn attempt [program noun verb]
  (-> program
      (assoc 1 noun)
      (assoc 2 verb)
      exec
      (get 0)))

(defn solve [program]
  (for [noun (range 100)
        verb (range 100)
        :when (= (attempt program noun verb) 19690720)]
    (+ (* noun 100) verb)))

(defn str->program [s]
  (->> (str/split s #",")
       (map str/trim)
       (remove empty?)
       (map parse-long)
       (into [])))

(defn -main
  [& _]
  (println (solve (str->program (slurp "large.in")))))
