(ns d01-1.core
  (:gen-class))

(def a ^:my-metadata [1 2 3])

(do
  (+ 1 1 2 2)
  (println "hello"))

(defn -main
  [& _]
  (println a)
  (println (meta a)))
