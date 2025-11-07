(ns d12-1.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [clojure.math.combinatorics :as comb]))

(def coords [:x :y :z])

(def empty-vec (apply hash-map (flatten (map #(vector % 0) coords))))

(defn add-vecs [v1 v2]
  (reduce #(assoc %1 %2 (+ (%2 v1 0) (%2 v2 0)))
          v1
          coords))

(deftest test-add-vecs
  (is (= {:smth 12 :x 13 :y 5 :z 0} (add-vecs {:smth 12 :x 1 :y 2} {:y 3 :x 12}))))

(defn str->vec [s]
  (let [[_ in-tag] (re-matches #"^\s*<(.*)>\s*$" s)]
    (assert in-tag)
    (reduce
     (fn [parsed-vec part]
       (let [[_ key val] (re-matches #"^\s*(\w+)\s*=(-?\d+)\s*$" part)
             sym-key (case key
                       "x" :x
                       "y" :y
                       "z" :z
                       (throw (ex-info "Unknown key" {:key key :part part :s s})))]
         (assert val)
         (assoc parsed-vec sym-key (parse-long val))))
     {}
     (str/split in-tag #"\s*,\s*"))))

(deftest test-str->vec
  (is (= {:x -1 :y 0 :z 23} (str->vec " <x=-1, y=0, z=23> "))))

(defn str->planets [s]
  (->> (str/split s #"\n")
       (map str/trim)
       (remove empty?)
       (map #(hash-map :id %1 :pos (str->vec %2) :vel empty-vec) (range))))

(defn vec->str [vec]
  (str
   "<"
   (str/join ", " (map #(str (str/replace-first % #"^:" "") " = " (% vec 0)) coords))
   ">"))

(deftest test-vec->str
  (is (= "<x = 1, y = 2, z = 0>" (vec->str {:x 1 :y 2}))))

(defn calc-next-vels [planets]
  (reduce
   (fn [next-planets cur-planet]
     (let [next-velocity
           (reduce
            (fn [next-velocity [coord other-planet]]
              (if (= (:id cur-planet) (:id other-planet))
                next-velocity
                (update next-velocity coord
                        (fn [prev-coord-velocity]
                          (let [cur-coord (coord (:pos cur-planet) 0)
                                other-coord (coord (:pos other-planet) 0)]
                            (+ (or prev-coord-velocity 0)
                               (compare (- other-coord cur-coord) 0)))))))

            (:vel cur-planet)
            (comb/cartesian-product coords planets))]
       (conj next-planets (assoc cur-planet :vel next-velocity))))
   []
   planets))

(deftest test-calc-next-vels
  (is (= [{:id "Ganymede", :pos {:x 3}, :vel {:x 1, :y 0, :z 0}}
          {:id "Callisto", :pos {:x 5}, :vel {:x -1, :y 0, :z 0}}]
         (calc-next-vels [{:id "Ganymede" :pos {:x 3} :vel {:x 0}}
                          {:id "Callisto" :pos {:x 5} :vel {:x 0}}]))))

(defn apply-vels [planets]
  (reduce
   #(conj %1 (assoc %2 :pos (add-vecs (:pos %2) (:vel %2))))
   []
   planets))

(deftest test-apply-vels
  (is (= [{:pos {:x 12 :y 0 :z 0}, :vel {:x 9}}]
         (apply-vels [{:pos {:x 3} :vel {:x 9}}]))))

(defn simulate [planets]
  (lazy-seq (cons planets (simulate (-> planets calc-next-vels apply-vels)))))

(deftest test-simulate
  (is (= [{:x 2,  :y  1  :z -3} {:x 1, :y -8 :z 0} {:x 3, :y -6 :z 1} {:x 2, :y  0 :z 4}]
         (map :pos (nth (simulate (str->planets "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>\n")) 10)))))

(defn calc-energy [planet]
  (reduce * (map (fn [key]
                   (let [vec (key planet {})]
                     (reduce #(+ %1 (abs (%2 vec 0))) 0 coords)))
                 [:pos :vel])))

(deftest test-calc-energy
  (is (= 60 (calc-energy {:pos {:x 1 :y 2 :z -3} :vel {:x 2 :y 2 :z -6}}))))

(defn solve [s steps]
  (let [planets (nth (simulate (str->planets s)) steps)
        energies (map calc-energy planets)
        energy (reduce + energies)]
    energy))

(deftest test-solve
  (is (= 1940
         (solve "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>" 100))))

(defn -main [& _]
  (println (solve (slurp  "./large.in") 1000)))
