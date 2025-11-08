(ns d14-1.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [clojure.math.combinatorics :as comb]))

(defn match->ingredient [[_ s_quantity ingredient]]
  {:ingredient (keyword ingredient) :cnt (parse-long s_quantity)})

(defn str->reaction [s]
  (let [[inp out] (str/split s #"=>")
        ingredients (vec (map match->ingredient (re-seq #"(\d+) (\w+)" inp)))
        {:keys [ingredient cnt]} (match->ingredient (re-find #"(\d+) (\w+)" out))]
    (assert (or (not= ingredient :FUEL) (= 1 cnt)))
    (assert (not= ingredient :ORE))
    (assert (= (count ingredients) (inc (count (re-seq #"," inp)))))
    {ingredient {:needs ingredients :cnt cnt}}))

(deftest test-str->reaction
  (is (= {:AB {:needs [{:ingredient :AZ, :cnt 32} {:ingredient :B, :cnt 4}], :cnt 1}}
         (str->reaction "32 AZ, 4 B => 1 AB"))))

(defn str->reactions [s]
  (let [reactions
        (->> (str/split-lines s)
             (map str/trim)
             (remove empty?)
             (map str->reaction))
        merged (reduce merge reactions)]
    (assert (= (count reactions) (count merged)))
    merged))

(defn reaction->dot [reaction]
  (let [node (name (first reaction))]
    (str/join "\n" (map #(str "    " node " -> " (name (:ingredient %)) " [label=\"" (:cnt %) "\"];")  (:needs (get reaction 1))))))

(defn reactions->dot [reactions]
  (str
   "digraph G{\n"
   (str/join "\n" (map (fn [reaction] (reaction->dot reaction)) reactions))
   "\n}"))

(defn -main [& _]
  (println (reactions->dot (str->reactions (slurp  "./large.in")))))
