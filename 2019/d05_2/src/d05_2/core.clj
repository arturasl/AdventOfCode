(ns d05-2.core
  (:gen-class)
  (:require [intcode.core :as code]))

(defn solve [s]
  (-> s
      code/str->memory
      code/init-program
      (code/to->stdin [5])
      code/exec
      (get :output)
      last))

(defn -main
  [& _]
  (println (solve (slurp  *in*))))
