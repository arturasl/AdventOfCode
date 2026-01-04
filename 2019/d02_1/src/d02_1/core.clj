(ns d02-1.core
  (:gen-class)
  (:require [intcode.core :as code]))

(defn solve [s]
  (-> (code/str->memory s)
      (assoc 1 12)
      (assoc 2 2)
      code/init-program
      code/exec
      (get-in [:memory 0])))

(defn -main
  [& _]
  (println (solve (slurp *in*))))
