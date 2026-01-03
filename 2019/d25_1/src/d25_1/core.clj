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

(defn robot-move [orig-program dir]
  (assert (or (nil? dir)
              (contains? #{:east :west :north :south} dir))
          (str dir))
  (let [program (if (nil? dir)
                  (-> orig-program
                      (assoc :robot {:items #{}}))
                  (str-to->stdin orig-program (name dir)))
        str-output (get-str-output program)
        str-output (str/replace str-output #"(?s).*and you are ejected back to the checkpoint.*?\n" "")
        matches (zipmap [:name :desc :doors :items]
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
                         (update :items str-list-to-set))]
    (-> program
        (assoc :output (finger/double-list))
        (assoc :room parsed-parts))))

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
                   "photons"
                   ; The giant electromagnet is stuck to you.  You can't move!!
                   "giant electromagnet"}
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
            ; If we drop, lets not try to pick it back as otherwise state key
            ; becomes very large.
            ; (update-in [:room :items] conj item)
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
  {:name (get-in program [:room :name])
   :robot-items (get-in program [:robot :items])})

(defn programs->grid [grid programs]
  (->> programs
       (map #(vector (program->grid-key %) %))
       (into grid)))

(defn print-grid-stats [grid]
  (let [uniq-msgs (sort
                   (distinct
                    (map #(get-in % [:room :desc])
                         (vals grid))))
        uniq-items (sort
                    (distinct
                     (sequence
                      cat
                      (map #(get-in % [:room :items])
                           (vals grid)))))
        uniq-rooms (sort
                    (distinct
                     (map #(get-in % [:room :name])
                          (vals grid))))]
    (println "")
    (println "### Unique messages (" (count uniq-msgs) ")")
    (doseq [msg uniq-msgs]
      (println msg))

    (println "")
    (println "### Unique items (" (count uniq-items) ")")
    (doseq [item uniq-items]
      (println item))

    (println "")
    (println "### Unique rooms (" (count uniq-rooms) ")")
    (doseq [room uniq-rooms]
      (println room))))

(defn search [starts]
  (loop [search-space (into (finger/double-list) starts)
         grid (programs->grid {} starts)
         its 1]
    (when (zero? (mod its 500))
      (println "Its:" its
               "Search space:" (count search-space)
               "Visited:" (count grid))
      (print-grid-stats grid))
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
    (print-grid-stats grid)))

(defn -main
  [& _]
  (solve (slurp *in*)))
