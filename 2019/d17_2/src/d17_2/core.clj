(ns d17-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [intcode.core :as code]))

(defn output-to-grid [output]
  (let [cells (->> output
                   (map char)
                   (apply str)
                   (str/split-lines)
                   (map vec)
                   vec)
        height (count cells)
        width (count (first cells))]
    {:height height
     :width width
     :cells cells}))

(defn read-grid [memory]
  (->> memory
       code/init-program
       code/exec
       :output
       output-to-grid))

(defn print-grid [{:keys [cells]}]
  (println (str/join "\n" (map str/join cells))))

(defn ins-to-bytes [ins]
  (let [ins-str-arr (map (fn [val]
                           (if (keyword? val)
                             (name val)
                             (str val)))
                         ins)]
    (conj (vec (map int (str/join "," ins-str-arr)))
          (int \newline))))

(deftest test-ins-to-bytes
  (is (= [65 44 66 44 49 48 44 76 10]
         (ins-to-bytes [:A :B 10 "L"])))
  (is (= [65 44 66 44 67 44 66 44 65 44 67 10]
         (ins-to-bytes [:A :B :C :B :A :C])))
  (is (= [82 44 56 44 82 44 56 10]
         (ins-to-bytes ["R" 8 "R" 8])))
  (is (= [82 44 52 44 82 44 52 44 82 44 56 10]
         (ins-to-bytes ["R" 4 "R" 4 "R" 8])))
  (is (= [76 44 54 44 76 44 50 10]
         (ins-to-bytes ["L" 6 "L" 2]))))

; L 4 L 4 L 6 R 10 L 6 L 4 L 4 L 6 R 10 L 6 L 12 L 6 R 10 L 6 R 8
; R 10 L 6 R 8 R 10 L 6 L 4 L 4 L 6 R 10 L 6 R 8 R 10 L 6 L 12 L 6
; R 10 L 6 R 8 R 10 L 6 L 12 L 6 R 10 L 6

; A A B C C A C B C B
; A: L 4 L 4 L 6 R 10 L 6
; B: L 12 L 6 R 10 L 6
; C: R 8 R 10 L 6

(defn solve [s]
  (let [memory (code/str->memory s)
        grid (read-grid memory)
        _ (print-grid grid)
        memory (assoc memory 0 2)
        program (code/exec
                 (code/to->stdin
                  (code/init-program memory)
                  (vec
                   (concat (ins-to-bytes [:A :A :B :C :C :A :C :B :C :B])
                           (ins-to-bytes ["L" 4 "L" 4 "L" 6 "R" 10 "L" 6])
                           (ins-to-bytes ["L" 12 "L" 6 "R" 10 "L" 6])
                           (ins-to-bytes ["R" 8 "R" 10 "L" 6])
                           [(int \n) (int \newline)]))))]
    (println (last (:output program)))))

(defn -main
  [& _]
  (solve (slurp  "./large.in")))
