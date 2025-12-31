(ns d09-2.core
  (:gen-class)
  (:require [intcode.core :as code]))

(defn solve [s]
  (-> s
      code/str->memory
      code/init-program
      (code/to->stdin [2])
      code/exec
      :output))

(defn -main
  [& _]
  (println (solve (slurp  "./large.in"))))
