(ns d24-1.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]))

(def ^:const coords-around
  [{:y -1 :x 0} {:y 1 :x 0}
   {:y 0 :x -1} {:y 0 :x 1}])

(defn add-coords [lhs rhs]
  (merge-with + lhs rhs))

(deftest test-add-coords
  (is (= {:y 14 :x 3} (add-coords {:y 5 :x 2} {:y 9 :x 1}))))

(defn str->state [s]
  (let [lines (->> s
                   str/split-lines
                   (map str/trim)
                   (remove empty?)
                   vec)
        height (count lines)
        _ (assert (not (zero? height)))
        width (count lines)
        _ (assert (every? #(= (count %) width) lines))
        bugs (into #{}
                   (for [y (range height)
                         x (range width)
                         :let [cell (get-in lines [y x])]
                         :when (= cell \#)]
                     {:y y :x x}))]
    {:height height
     :width width
     :bugs bugs}))

(defn calc-next-state [s]
  (assoc
   s :bugs (into
            #{} (for [y (range (:height s))
                      x (range (:width s))
                      :let [coord {:y y :x x}
                            is-bug (contains? (:bugs s) coord)
                            num-bugs-around (->> coords-around
                                                 (map (fn [dc] (add-coords dc coord)))
                                                 (map (fn [c] (contains? (:bugs s) c)))
                                                 (filter identity)
                                                 count)]
                      :when (or (and is-bug
                                     (= 1 num-bugs-around))
                                (and (not is-bug)
                                     (contains? #{1 2} num-bugs-around)))]
                  coord))))

(defn find-repeating [s]
  (loop [state s
         visited #{}]
    (if (contains? visited state)
      state
      (recur (calc-next-state state) (conj visited state)))))

(defn calc-biodiversity [s]
  (reduce +
          (map (fn [{:keys [y x]}]
                 (bit-shift-left 1 (+ (* y (:width s)) x)))
               (:bugs s))))

(defn solve [s]
  (let [state (str->state s)
        repeated-state (find-repeating state)]
    (calc-biodiversity repeated-state)))

(defn -main
  [& _]
  (println (solve (slurp *in*))))
