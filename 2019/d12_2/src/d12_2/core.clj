(ns d12-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [clojure.math.numeric-tower :as num-tower]
            [clojure.math.combinatorics :as comb]))

(def coords [:x :y :z])

(def empty-vec (zipmap coords (repeat 0)))

(defn add-vecs [v1 v2]
  (reduce (fn [vec coord] (update vec coord (fnil + 0 0) (coord v2)))
          v1
          coords))

(deftest test-add-vecs
  (is (= {:smth 12 :x 13 :y 5 :z 0} (add-vecs {:smth 12 :x 1 :y 2} {:y 3 :x 12}))))

(defn str->vec [s]
  (into {}
        (for [[_ k v] (re-seq #"(\w+)\s*=\s*(-?\d+)" s)]
          [(keyword k) (parse-long v)])))

(deftest test-str->vec
  (is (= {:x -1 :y 0 :z 23} (str->vec " <x=-1, y=0, z=23> "))))

(defn str->planets [s]
  (->> (str/split-lines s)
       (map str/trim)
       (remove empty?)
       (map #(hash-map :id %1 :pos (str->vec %2) :vel empty-vec) (range))))

(defn vec->str [vec]
  (str
   "<"
   (str/join ", " (map #(str (name %) " = " (% vec 0)) coords))
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
  (map #(update % :pos add-vecs (:vel %)) planets))

(deftest test-apply-vels
  (is (= [{:pos {:x 12 :y 0 :z 0}, :vel {:x 9}}]
         (apply-vels [{:pos {:x 3} :vel {:x 9}}]))))

(defn simulate [planets]
  (lazy-seq (cons planets (simulate (-> planets calc-next-vels apply-vels)))))

(deftest test-simulate
  (is (= [{:x 2,  :y  1  :z -3} {:x 1, :y -8 :z 0} {:x 3, :y -6 :z 1} {:x 2, :y  0 :z 4}]
         (map :pos (nth (simulate (str->planets "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>\n")) 10)))))

(defn find-loop [planets]
  (some
   (fn [[i cur-plannets]] (and (= planets cur-plannets) (inc i)))
   (map vector (range)
        (drop 1 (simulate planets)))))

(deftest test-find-loop
  (is (= 2772
         (find-loop (str->planets "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>\n")))))

(defn leave-only-coord [plannet coord]
  (assoc plannet :pos (assoc empty-vec coord (get-in plannet [:pos coord]))))

(deftest test-leave-only-coord
  (is (= {:pos {:x 0 :y 2 :z 0}}
         (leave-only-coord {:pos {:x 1 :y 2 :z 3}} :y))))

(defn find-loop-fast [planets]
  (reduce
   num-tower/lcm
   (map (fn [coord] (find-loop (map #(leave-only-coord % coord) planets))) coords)))

(deftest test-find-loop-fast
  (is (= 2772
         (find-loop-fast (str->planets "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>"))))
  (is (= 4686774924
         (find-loop-fast (str->planets "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>")))))

(defn solve [s]
  (find-loop-fast (str->planets s)))

(defn -main [& _]
  (println (solve (slurp  "./large.in"))))
