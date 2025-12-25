(ns d22-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]))

(defn str->op [s]
  (let [deal (re-matches #"^deal with increment (\d+)$" s)
        cut (re-matches #"^cut (-?\d+)$" s)]
    (cond
      (not (nil? deal)) {:op :deal :amount (parse-long (second deal))}
      (not (nil? cut)) {:op :cut :amount (parse-long (second cut))}
      :else (do
              (assert (= s "deal into new stack"))
              {:op :reverse}))))

(deftest test-str->op
  (is (= {:op :deal :amount 7}
         (str->op "deal with increment 7")))
  (is (= {:op :deal :amount 78}
         (str->op "deal with increment 78")))
  (is (= {:op :cut :amount -1}
         (str->op "cut -1")))
  (is (= {:op :cut :amount 12}
         (str->op "cut 12")))
  (is (= {:op :reverse}
         (str->op "deal into new stack"))))

(defn str->ops [s]
  (->> s
       (str/split-lines)
       (map str/trim)
       (remove empty?)
       (map str->op)))

(def inv-mod-prime
  (memoize
   (fn [n p]
     (loop [matrix [[1N 0N n] [0N 1N p]]]
       (if (zero? (get-in matrix [1 2]))
         (mod (get-in matrix [0 0]) p)
         (let [div (quot (get-in matrix [0 2]) (get-in matrix [1 2]))]
           (recur [(get matrix 1)
                   (vec (map #(- %1 (* %2 div))
                             (get matrix 0)
                             (get matrix 1)))])))))))

(deftest test-inv-mod-prime
  (is (= 0 (inv-mod-prime 0 7)))
  (is (= 5 (inv-mod-prime 3 7)))
  (is (= 6 (inv-mod-prime 6 7)))
  (is (= 108506422313517 (inv-mod-prime 2020 119315717514047))))

(defn follow-rev
  ([total-cards ops]
   (let [[mul oft] (follow-rev [1N 0N] total-cards ops)]
     [(mod mul total-cards) (mod oft total-cards)]))
  ([[mul oft] total-cards ops]
   (reduce
    (fn [[mul oft] {:keys [op amount]}]
      (case op
        :reverse [(- mul) (dec (- oft))]
        :cut [mul (+ oft amount)]
        :deal (let [inv (inv-mod-prime amount total-cards)]
                [(* mul inv) (* oft inv)])
        (throw (ex-info "Unknown op" {:op op}))))
    [mul oft]
    ops)))

(defn pow-mod [v p m]
  (if (zero? p) 1N
      (let [half-result (pow-mod v (quot p 2) m)
            even-result (mod (* half-result half-result) m)]
        (if (zero? (mod p 2))
          even-result
          (mod (* even-result v) m)))))

(deftest test-pow-mod
  (is (= 3 (pow-mod 3 1 11)))
  (is (= 9 (pow-mod 3 2 11)))
  (is (= 2 (pow-mod 3 2 7)))
  (is (= 27 (pow-mod 3 3 30)))
  (is (= 10 (pow-mod 3 3 17))))

; 0 - 1
; 1 - x + 1
; 2 - x * x + x + 1
; 3 - x * x * x + x * x + x + 1
; 4 - x * x * x * x + x * x * x + x * x + x + 1
; (defn rep-pow-mod [x n m]
;   (if (zero? n) 1
;       (mod (inc (* (rep-pow-mod x (dec n) m) x)) m)))

; (x ** 1) * (1) + (1) = x + 1
; (x ** 2)(x + 1) + (x + 1) = x ** 3 + x ** 2 + x + 1
; (x ** 4)(x ** 3 + x ** 2 + x + 1) + (x ** 3 + x ** 2 + x + 1) = x ** 7 + x ** 6 ...
; (x ** 8)(x ** 7 + ...) + (x ** 7 + ...) = x ** 15 + x ** 14 ...

; f(n) = (x ** ceil(n / 2)) * (f(floor(n / 2) - 1 if even) + f(floor(n / 2))
; f(4) = (x ** 2) * (x ** 2 + x + 1) + (x ** 2 + x + 1)
;      = x ** 4 * x ** 3 + x ** 2 + x ** 2 + x + 1
; f(5) = (x ** 3) * (x ** 2 + x + 1) + (x ** 2 + x + 1)
(defn rep-pow-mod [x n m]
  (if (zero? n) 1
      (let [is_even (even? n)
            divfloor (quot n 2)
            divceil (+ divfloor (if is_even 0 1))
            a (pow-mod x divceil m)
            b (rep-pow-mod x divfloor m)]
        (mod (+ (* a (- b (if is_even 1 0))) b) m))))

(deftest test-rep-pow-mod
  (is (= 1 (rep-pow-mod 3 0 1000)))
  (is (= 4 (rep-pow-mod 3 1 1000)))
  (is (= 13 (rep-pow-mod 3 2 1000)))
  (is (= 40 (rep-pow-mod 3 3 1000)))
  (is (= 121 (rep-pow-mod 3 4 1000))))

(defn solve [s]
  (let [ops (reverse (str->ops s))
        end-pos 2020N
        reps 101741582076661N
        total-cards 119315717514047N
        [mul oft] (follow-rev total-cards ops)]
    (mod
     (+
      (* end-pos (pow-mod mul reps total-cards))
      (* oft (rep-pow-mod mul (dec reps) total-cards)))
     total-cards)))

(defn -main
  [& _]
  (println (solve (slurp *in*))))
