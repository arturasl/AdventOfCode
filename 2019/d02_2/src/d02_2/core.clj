(ns d02-2.core
  (:gen-class)
  (:require [intcode.core :as code]))

(defn attempt [mem noun verb]
  (-> mem
      (assoc 1 noun)
      (assoc 2 verb)
      code/init-program
      code/exec
      (get-in [:memory 0])))

(defn solve [mem]
  (for [noun (range 100)
        verb (range 100)
        :when (= (attempt mem noun verb) 19690720)]
    (+ (* noun 100) verb)))

(defn -main
  [& _]
  (println (solve (code/str->memory (slurp *in*)))))
