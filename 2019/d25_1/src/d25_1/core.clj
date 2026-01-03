(ns d25-1.core
  (:gen-class)
  (:require [intcode.core :as code]
            [clojure.string :as str]
            [clojure.data.finger-tree :as finger]))

(defn str-list-to-set [s]
  (if (nil? s) #{}
      (let [lines (vec (rest (str/split-lines s)))
            _ (assert (not-empty lines) s)
            kwds (vec (map keyword (map #(str/replace % #"- " "") lines)))
            set-kwds (into #{} kwds)
            _ (assert (= (count kwds) (count set-kwds)))]
        set-kwds)))

(defn str-to->stdin [program s]
  (code/exec (code/to->stdin program (vec (map int (str s \newline))))))

(defn get-str-output [program]
  [(assoc program :output (finger/double-list))
   (apply str (map char (:output program)))])

(defn robot-move [program dir]
  (assert (or (nil? dir)
              (contains? #{:east :west :north :south} dir))
          (str dir))
  (let [program (if (nil? dir) program
                    (str-to->stdin program (name dir)))
        [program str-output] (get-str-output program)
        matches (zipmap [:room :desc :doors :items]
                        (rest (re-matches #"(?s)^\n*(== [^=]+ ==)\n+([^\n]+)\n+(Doors here lead:\n(?:[^\n]+\n)+)?\n*(Items here:\n(?:[^\n]+\n)+)?\n*Command\?\n+$" str-output)))
        _ (assert (not (nil? (:room matches))) str-output)
        parsed-parts (-> matches
                         (update :room #(str/replace % #"^=+ *| *=+$" ""))
                         (update :doors str-list-to-set)
                         (update :items str-list-to-set))]
    [program parsed-parts]))

(defn robot-take [program item]
  (let [program (str-to->stdin program (str "take " (name item)))
        [program str-output] (get-str-output program)
        matches (zipmap [:took]
                        (rest (re-matches #"(?s)^\nYou take (?:the|a|an) ([^\.]+)\.\n+Command\?\n+$" str-output)))
        _ (assert (not (nil? (:took matches))) str-output)
        parsed-parts (-> matches
                         (update :took keyword))
        _ (assert (= (:took parsed-parts) item))]
    [program parsed-parts]))

(defn solve [s]
  (let [commands [:east :east :east :west :south :north :west :north :north :east]
        program (->> s code/str->memory code/init-program code/exec)
        [program init-output] (robot-move program nil)]
    (println init-output)
    (let [p (reduce
             (fn [program command]
               (let [[program output] (robot-move program command)]
                 (println output)
                 program))
             program
             commands)
          [p output] (robot-take p :cake)
          _ (println output)])))

(defn -main
  [& _]
  (solve (slurp *in*)))
