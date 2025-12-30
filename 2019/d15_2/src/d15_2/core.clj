(ns d15-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.data.finger-tree :as finger]
            [intcode.core :as code]))

(defn grid->str [grid robot]
  (let [ys (filter (comp not nil?) (conj (map :y (keys grid)) (:y robot)))
        xs (filter (comp not nil?) (conj (map :x (keys grid)) (:x robot)))
        [min-y max-y] (map #(apply % ys) [min max])
        [min-x max-x] (map #(apply % xs) [min max])
        height (inc (- max-y min-y))
        width (inc (- max-x min-x))
        drawn (reduce (fn [drawn-so-far [y x]]
                        (assoc-in drawn-so-far [y x]
                                  (let [pos {:x (+ x min-x) :y (+ y min-y)}]
                                    (if (= pos robot) "r"
                                        (case (get grid pos :unknown)
                                          :wall "#"
                                          :unknown "?"
                                          :oxygen "_"
                                          :empty " ")))))
                      (into [] (repeat height (into [] (repeat width " "))))
                      (for [y (range height) x (range width)] [y x]))]
    (str/join "\n" (map str/join drawn))))

(defn add [pos1 pos2]
  (let [coords [:y :x]]
    (zipmap coords
            (map #(+ (% pos1) (% pos2)) coords))))

(def moves [{:id 1 :rel-pos {:x 0 :y -1}},
            {:id 2 :rel-pos {:x 0 :y 1}},
            {:id 3 :rel-pos {:x -1 :y 0}},
            {:id 4 :rel-pos {:x 1 :y 0}}])

(defn find-closest [grid start search-for]
  (loop [space (finger/double-list {:poses [start] :path []})
         visited #{start}
         max-poses 1]
    (if (empty? space)
      (do
        (println (dec max-poses))
        nil)
      (let [{:keys [poses path] :as state} (first space)
            pos (last poses)
            space (rest space)
            around (map #(assoc % :abs-pos (add (:rel-pos %) pos)) moves)
            valid-around (remove (fn [{:keys [abs-pos]}]
                                   (or (contains? visited abs-pos)
                                       (= (get grid abs-pos) :wall)))
                                 around)
            next-visited (into visited (map :abs-pos valid-around))
            next-space (apply conj space (map #(hash-map
                                                :poses (conj poses (:abs-pos %))
                                                :path (conj path (:id %)))
                                              valid-around))]
        (if (= (get grid pos :unknown) search-for)
          state
          (recur next-space
                 next-visited
                 (max max-poses (count (:poses (last next-space))))))))))

(defn resolve-grid [program]
  (loop [program program
         robot {:x 0 :y 0}
         grid {robot :empty}]
    (println)
    (println (grid->str grid robot))
    (let [found-closest (find-closest grid robot :unknown)]
      (if (nil? found-closest)
        grid
        (let [ids-till-unknown (:path found-closest)
              poses-till-unknown (:poses found-closest)
              next-program (-> program
                               (code/to->stdin ids-till-unknown)
                               code/exec)
              _ (assert (every? #(= % 1) (butlast (:output next-program)))
                        (str (:output next-program) robot poses-till-unknown ids-till-unknown))
              out (last (:output next-program))
              next-grid (assoc grid (last poses-till-unknown)
                               (case out
                                 0 :wall
                                 1 :empty
                                 2 :oxygen))
              next-robot (if (zero? out)
                           (last (butlast poses-till-unknown))
                           (last poses-till-unknown))]
          (recur (assoc next-program
                        :output
                        (finger/double-list))
                 next-robot next-grid))))))

(defn solve [s]
  (let [grid
        (-> s
            code/str->memory
            code/init-program
            code/exec
            resolve-grid)
        found-oxygen (find-closest grid {:x 0 :y 0} :oxygen)]
    (find-closest grid (last (:poses found-oxygen)) :does-not-exist)))

(defn -main
  [& _]
  (solve (slurp  "./large.in")))
