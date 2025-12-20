(ns d18-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [clojure.data.finger-tree :as finger]
            [clojure.data.priority-map :refer [priority-map]]))

(defn str->grid [s]
  (let [grid (->> s
                  (str/split-lines)
                  (map str/trim)
                  (remove empty?)
                  (map
                   (fn [line]
                     (map
                      (fn [char]
                        (cond
                          (= char \#) {:type :wall}
                          (= char \.) {:type :empty}
                          (= char \@) {:type :self}
                          (Character/isUpperCase char)
                          {:type :door :val char}
                          (Character/isLowerCase char)
                          {:type :key :val (Character/toUpperCase char)}
                          :else (throw (ex-info "Unknown symbol" {:char char}))))
                      line)))
                  (map vec)
                  vec)
        height (count grid)
        width (count (get grid 0))
        _ (assert (every? #(= (count %) width) grid) "Inconsistent width")
        keys-positions (into {}
                             (for [y (range height)
                                   x (range width)
                                   :let [cell (get-in grid [y x])]
                                   :when (= (:type cell) :key)]
                               [(:val cell) [y x]]))
        starts (for [y (range height)
                     x (range width)
                     :let [cell (get-in grid [y x])]
                     :when (= (:type cell) :self)]
                 [y x])
        _ (assert (= (count starts) 1) "There should be only one start")
        [start-y start-x] (first starts)
        starts (into #{} (for [dy (range -1 2) dx (range -1 2)
                               :when (and (not (zero? dy))
                                          (not (zero? dx)))]
                           [(+ start-y dy) (+ start-x dx)]))]
    {:height height
     :width width
     :keys (into #{} (keys keys-positions))
     :key-positions  keys-positions
     :starts starts
     :grid (reduce
            (fn [grid [y x]]
              (assoc-in grid [y x] {:type :empty}))
            (reduce (fn [grid [dy dx]]
                      (assoc-in grid [(+ dy start-y) (+ dx start-x)] {:type :wall}))
                    grid
                    (for [dy (range -1 2) dx (range -1 2)] [dy dx]))
            starts)}))

(deftest test-str->grid
  (is (=
       {:height 4
        :width 3
        :keys #{\A}
        :key-positions {\A [3 0]}
        :starts #{[0 0] [0 2] [2 0] [2 2]}
        :grid [[{:type :empty} {:type :wall} {:type :empty}]
               [{:type :wall} {:type :wall} {:type :wall}]
               [{:type :empty} {:type :wall} {:type :empty}]
               [{:type :key :val \A} {:type :door :val \A} {:type :wall}]]}
       (str->grid "...\n.@.\n...\naA#"))))

(defn shortest-path [grid start can-use-keys any-of-dest]
  (loop [states (finger/double-list {:pos start :dist 0})
         visited #{start}]
    (let [cur-state (first states)
          states (rest states)]
      (cond
        (nil? cur-state) nil
        (contains? any-of-dest (:pos cur-state)) cur-state
        :else (let [[cur-y cur-x] (:pos cur-state)
                    next-poses (into #{}
                                     (for [dy (range -1 2)
                                           dx (range -1 2)
                                           :let [cell-pos [(+ dy cur-y) (+ dx cur-x)]
                                                 cell (get-in grid cell-pos)
                                                 cell-type (:type cell)]
                                           :when (and
                                                  (not (nil? cell))
                                                  (not (zero? (bit-xor
                                                               (if (zero? dy) 1 0)
                                                               (if (zero? dx) 1 0))))
                                                  (not= cell-type :wall)
                                                  (or (and (not= cell-type :door)
                                                           (not= cell-type :key))
                                                      (contains? can-use-keys (:val cell)))
                                                  (not (contains? visited cell-pos)))]
                                       cell-pos))]
                (recur
                 (into states (map (fn [p]
                                     {:pos p
                                      :dist (inc (:dist cur-state))})
                                   next-poses))
                 (into visited next-poses)))))))

(def ^:const max-dist-ever 1000000000)

(if (<= 1 2) "ok" "nok")

(defn solve [s]
  (let [grid (str->grid s)
        start-state {:keys #{} :poses (:starts grid)}]
    (loop [search-space (priority-map start-state 0)
           globals {:best max-dist-ever
                    :its 0
                    :visited {start-state 0}}]
      (if (empty? search-space) (:best globals)
          (let [[cur-state cur-dist] (peek search-space)
                cur-keys (:keys cur-state)
                search-space (pop search-space)
                globals (update globals :its inc)]
            (when (zero? (mod (:its globals) 100))
              (binding [*out* *err*]
                (println "best: " (:best globals)
                         "its:" (:its globals)
                         "search-space size:" (count search-space)
                         "cur-state:" cur-state
                         "cur-dist:" cur-dist
                         "visited:" (count (:visited globals)))))
            (cond
              (>= cur-dist (:best globals)) (recur search-space globals)
              (= cur-keys (:keys grid)) (recur search-space (assoc globals :best cur-dist))
              :else (let [cur-poses (:poses cur-state)
                          next-states (for [[key key-pos] (:key-positions grid)
                                            :when (not (contains? cur-keys key))
                                            :let [shortest (shortest-path
                                                            (:grid grid)
                                                            key-pos
                                                            cur-keys
                                                            cur-poses)]
                                            :when (not (nil? shortest))
                                            :let [next-keys (conj cur-keys key)
                                                  next-poses (conj
                                                              (disj cur-poses
                                                                    (:pos shortest))
                                                              key-pos)
                                                  next-dist (+ cur-dist (:dist shortest))
                                                  next-state {:keys next-keys :poses next-poses}]
                                            :when (< next-dist (get (:visited globals) next-state max-dist-ever))]
                                        [next-state next-dist])
                          next-states (->> next-states
                                           (group-by first)
                                           (map (fn [[state grouped]] [state (map second grouped)]))
                                           (map (fn [[state dists]] [state (reduce min dists)])))
                          globals (update globals :visited #(into % next-states))
                          search-space (into search-space next-states)]
                      (recur search-space globals))))))))

(deftest test-solve
  (is (= max-dist-ever
         (solve (str/join \newline
                          ["#######"
                           "#a.#Zd#"
                           "##...##"
                           "##.@.##"
                           "##...##"
                           "#cB#Ab#"
                           "#######"]))))
  (is (= 9
         (solve (str/join \newline
                          [".#b###N"
                           "q..#..B"
                           "...#.#."
                           "###@###"
                           "...#.#."
                           ".Q##..."
                           "..n#..."]))))
  (is (= 8
         (solve (str/join \newline
                          ["#######"
                           "#a.#Cd#"
                           "##...##"
                           "##.@.##"
                           "##...##"
                           "#cB#Ab#"
                           "#######"]))))
  (is (= 32
         (solve (str/join \newline
                          ["#############"
                           "#DcBa.#.GhKl#"
                           "#.###...#I###"
                           "#e#d#.@.#j#k#"
                           "###C#...###J#"
                           "#fEbA.#.FgHi#"
                           "#############"]))))
  (is (= 72
         (solve (str/join \newline
                          ["#############"
                           "#g#f.D#..h#l#"
                           "#F###e#E###.#"
                           "#dCba...BcIJ#"
                           "#####.@.#####"
                           "#nK.L...G...#"
                           "#M###N#H###.#"
                           "#o#m..#i#jk.#"
                           "#############"])))))

(defn -main
  [& _]
  (println (solve (slurp *in*))))
