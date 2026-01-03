(ns d25-1.core
  (:gen-class)
  (:require [intcode.core :as code]
            [clojure.string :as str]
            [clojure.data.finger-tree :as finger]
            [clojure.math.combinatorics :as comb]))

(defn subsets [els]
  (map #(into #{} %)
       (comb/subsets (vec els))))

(defn str-list-to-set [s]
  (if (nil? s) #{}
      (let [lines (vec (rest (str/split-lines s)))
            _ (assert (not-empty lines) s)
            items (vec (map #(str/replace % #"- " "") lines))
            set-items (into #{} items)
            _ (assert (= (count items) (count set-items)))]
        set-items)))

(def str-to->stdin
  (memoize
   (fn [program s]
     (assoc
      (code/exec (code/to->stdin program (vec (map int (str s \newline)))))
      :input
      (finger/double-list)))))

(defn get-str-output [program]
  (str/join "" (map char (:output program))))

(defn add-coords [lhs rhs]
  (merge-with + lhs rhs))

(defn robot-move [orig-program dir]
  (assert (or (nil? dir)
              (contains? #{:east :west :north :south} dir))
          (str dir))
  (let [program (if (nil? dir)
                  (-> orig-program
                      (assoc :room {:coords {:y 0 :x 0}})
                      (assoc :robot {:items #{}}))
                  (str-to->stdin orig-program (name dir)))
        str-output (get-str-output program)]
    (if (not (nil? (str/index-of str-output "and you are ejected back to the checkpoint")))
      orig-program
      (let [matches (zipmap [:name :desc :doors :items]
                            (rest (re-matches
                                   (re-pattern
                                    (str
                                     "(?s)^\n*"
                                     "(== [^=]+ ==)\n+"
                                     "([^\n]+)\n+"
                                     "(Doors here lead:\n(?:[^\n]+\n)+)?\n*"
                                     "(Items here:\n(?:[^\n]+\n)+)?\n*"
                                     "Command\\?\n+$"))
                                   str-output)))
            _ (assert (not (nil? (:name matches))) str-output)
            parsed-parts (-> matches
                             (update :name #(str/replace % #"^=+ *| *=+$" ""))
                             (update :doors str-list-to-set)
                             (update :doors #(into #{} (map keyword %)))
                             (update :items str-list-to-set))
            coords (add-coords (get-in program [:room :coords])
                               (get {:north {:y -1 :x 0}
                                     :south {:y 1 :x 0}
                                     :east {:y 0 :x 1}
                                     :west {:y 0 :x -1}}
                                    dir))]
        (-> program
            (assoc :output (finger/double-list))
            (assoc :room parsed-parts)
            (assoc-in [:room :coords] coords))))))

(defn robot-take [program item]
  (let [room-has (get-in program [:room :items])]
    (assert (contains? room-has item)
            (str "Can not take: " item ", Room has: " room-has)))
  (if (contains? #{; You're launched into space! Bye!
                   "escape pod"
                   ; The molten lava is way too hot! You melt!
                   "molten lava"
                   ; :/
                   "infinite loop"
                   ; It is suddenly completely dark! You are eaten by a Grue!
                   "photons"}
                 item) program
      (let [program (str-to->stdin program (str "take " item))
            str-output (get-str-output program)
            matches (zipmap [:took]
                            (rest (re-matches #"(?s)^\nYou take (?:the|a|an) ([^\.]+)\.\n+Command\?\n+$" str-output)))
            _ (assert (not (nil? (:took matches))) str-output)
            _ (assert (= (:took matches) item))]
        (-> program
            (assoc :output (finger/double-list))
            (update-in [:room :items] disj item)
            (update-in [:robot :items] conj item)))))

(defn robot-drop [program item]
  (let [robot-has (get-in program [:robot :items])]
    (assert (contains? robot-has item)
            (str "Can not drop: " item ", Robot has:" robot-has)))
  (if (contains? #{} item) program
      (let [program (str-to->stdin program (str "drop " item))
            str-output (get-str-output program)
            matches (zipmap [:took]
                            (rest (re-matches #"(?s)^\nYou drop (?:the|a|an) ([^\.]+)\.\n+Command\?\n+$" str-output)))
            _ (assert (not (nil? (:took matches))) str-output)
            _ (assert (= (:took matches) item))]
        (-> program
            (assoc :output (finger/double-list))
            (update-in [:room :items] conj item)
            (update-in [:robot :items] disj item)))))

(defn robot-ensure-has [program items]
  (assert (set? items))
  (reduce
   (fn [acc-program item]
     (if (contains? (get-in acc-program [:robot :items]) item)
       (robot-drop acc-program item)
       (robot-take acc-program item)))
   program
   items))

(defn possible-item-subsets [program]
  (subsets (concat
            (get-in program [:room :items])
            (get-in program [:robot :items]))))

(defn get-programs-around [program]
  (let [doors (get-in program [:room :doors])
        around (map #(robot-move program %) doors)
        with-items (sequence
                    cat
                    (map (fn [program]
                           (map #(robot-ensure-has program %)
                                (possible-item-subsets program)))
                         around))]
    with-items))

(defn program->grid-key [program]
  {:y (get-in program [:room :coords :y])
   :x (get-in program [:room :coords :x])
   :items (get-in program [:robot :items])})

(defn programs->grid [grid programs]
  (->> programs
       (map #(vector (program->grid-key %) %))
       (into grid)))

(defn search [starts]
  (loop [search-space (into (finger/double-list) starts)
         grid (programs->grid {} starts)
         its 1]
    (when (zero? (mod its 100))
      (println "Its:" its
               "Search space:" (count search-space)
               "Visited:" (count grid)))
    (if (empty? search-space) {:grid grid}
        (let [cur (first search-space)
              its (inc its)
              search-space (rest search-space)
              next-programs (remove
                             #(contains? grid (program->grid-key %))
                             (get-programs-around cur))]
          (recur (into search-space next-programs)
                 (programs->grid grid next-programs)
                 its)))))

(defn solve [s]
  (let [program (-> s
                    code/str->memory
                    code/init-program
                    code/exec
                    (robot-move nil))
        grid (:grid (search [program]))]
    (println "")
    (println "### Unique messages")
    (doseq [msg (distinct
                 (map #(get-in % [:room :desc])
                      (vals grid)))]
      (println msg))

    (println "")
    (println "### Unique items")
    (doseq [item (distinct
                  (sequence
                   cat
                   (map #(get-in % [:room :items])
                        (vals grid))))]
      (println item))

    (println "")
    (println "### Unique rooms")
    (doseq [item (distinct
                  (map #(get-in % [:room :name])
                       (vals grid)))]
      (println item))))

(defn -main
  [& _]
  (solve (slurp *in*)))
