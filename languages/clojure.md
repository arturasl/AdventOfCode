# Variables

```clj
(def a 123)
(def ^:const pi 3.14) ; Constant.

; Create new scoped variables.
(let [a 1
      b (inc a)
      a b]
  (println a b)

; Dynamic binding (the `*var*` is convetion called earmuffins) -- thread local
; variables that can be changed anywhere in the stack.
(def ^:dynamic *a* 1)
(defn inca [] (set! *a* (inc *a*)))
(binding [*a* 2]
  (inca)
  (println *a*)) ; 2
```

# Conditions

```clj
(if (<= 1 2) "ok" "nok")
(when (<= 1 2) "ok") ; else defaults to nil.
```

```clj
(cond
  (< -1 0) "negative"
  (> -1 0) "positive"
  :else "zero")
```

```clj
(case (+ 2 3)
 3 "nok"
 5 "ok"
 "default")
```

```clj
; Note that < can accept multiple arguments making it easier to create bound
; checks.
(<= 0 y (dec height))
```

# Loops

```clj
; Iterate over items returning nil
(doseq [[i item] (map-indexed vector [9 8 7])]
  (println i ": " item))
; 0 9
; 1 8
; 2 7
```

```clj
; Produce a lazy sequence from given iterator Cartesian product (similar to
; map/reduce/let combinations).
(let [cur-y 12 cur-x 15 grid {11 {16 true}}]
  (for [dy (range -1 2)
        dx (range -1 2)
        :let [y (+ cur-y dy)
              x (+ cur-x dx)]
        :when (and (<= 0 y 100)
                   (<= 0 x 100))
        :let [cell (get-in grid [y x])]
        :when cell]
    {:y y :x x}))
```

```clj
; A simple way to define a `recur` context.
(loop [a 3]
  (if (zero? a) 0
      (do (println a)
          (recur (dec a)))))
; 3
; 2
; 1
```

# Errors

```clj
(throw (ex-info "Unknown opcode" {:opcode opcode}))))
(assert (not= 1 2) (str "Unknown opcode: " opcode))
```

# Functions

```clj
(defn func [arg] (println arg))
```

```clj
; Private function.
(defn- func [arg] (println arg))
```

```clj
; Multi arg function.
(defn func
  ([onearg] (println "one arg variant"))
  ([one two] (println "two arg variant")))
```

```clj
; Anonymous function
(#(println % %2) 1 2))
; Same as
(#(println %1 %2) 1 2))
; Same as
((fn [a1 a2] (println a1 a2)) 1 2))
```

```clj
; Memorization.
(def adder (memoize (fn [x y] (+ x y))))
```

# Containers

```clj
; Append to list/vector/map/set.
(= (conj '(1 2 3) 4) '(4 1 2 3))
(= (conj [1 2 3] 4) [1 2 3 4])
(= (conj {:a 1 :b 2} [:c 3]) {:a 1 :b 2 :c 3})
(= (conj #{1 2 3} 4) #{1 2 3 4})
(= (into #{} [1 2 3]) #{1 2 3}) ; conj multiple items
```

```clj
; Insert.
(= (assoc {:a 1 :b 2} :c 3) {:a 1 :b 2 :c 3})
(= (assoc [1 2 3] 1 9) [1 9 3])
(= (assoc-in [1 [2] 3] [1 0] 9) [1 [9] 3]) ; Dive into elements.
```

```clj
; Accessors.
(= (:a {:a 1}) 1) ; Map specific
(= (get {:a 1} :b :default-value) :default-value)
(= (get [1 2] 1) 2)
(= (get-in [1 [2]] [1 0]) 2) ; Dive into elements.
```

```clj
; Delete
(= (dissoc {:a 1 :b 2} :b) {:a 1}) ; Map specific
```

```clj
; Simple checks.
(= (contains? #{1 2 3} 1) true)
(= (empty? #{1 2 3}) false)
(= (not-empty #{1 2 3}) #{1 2 3}) ; No question mark as returns collection itself.
(= (= (count #{1 2 3}) 3) true)

```

```clj
; Update with a function
(= (update {:a 1 :b 1} :a inc) {:a 2 :b 1})
(= (update-in {:a {:b 1}} [:a :b] inc) {:a {:b 2}}) ; Dive into elements.
```

```clj
; Map specific
(= (vals {:a 1 :b 2}) [1 2])
(= (keys {:a 1 :b 2}) [:a :b])
(= (update-vals {:a 1 :b 2} inc) {:a 2 :b 3})
(= (select-keys {:a 1 :b 2} [:a]) {:a 1})
(= (merge {:a 1 :b 2} {:b 3 :c 4}) {:a 1 :b 3 :c 4})
(= (merge-with + {:a 1 :b 2} {:b 3 :c 4}) {:a 1 :b 5 :c 4})
(= (group-by first [[:a 1] [:a 2] [:b 3]]) {:a [[:a 1] [:a 2]] :b [[:b 3]]}
(= (frequencies [1 1 2 2 2]) {1 2 2 3}
(= (zipmap (range) [3 4]) {0 3 1 4})
```

```clj
; Sequences
(= (map inc [1 2 3]) [2 3 4])
(= (remove even? [1 2 3]) [1 3])
(= (filter even? [1 2 3]) [2])
(= (reduce (fn [acc cur] (assoc acc cur (inc cur))) {} [1 2 3])
   {1 2 2 3 3 4})
(= (take 2 [1 2 3]) [1 2])
(= (drop 1 [1 2 3]) [2 3])
(= (rest [1 2 3]) [2 3])
(= (distinct [1 1 3]) [1 3])
(= (first [1 2 3]) 1)
(= (second [1 2 3]) 2)
(= (last [1 2 3]) 3)
(= (nth [1 2 3] 1) 2)
(= (sequence cat [[1 [2]] [3]]) [1 [2] 3]) ; Flatten single level
(= (concat [1] [2 3] [4]) [1 2 3 4])
```

Convert vector into mutable (transient; as a performance optimization if
multiple changes are needed), insert a value into it and convert back
to an immutable version. Note that most operations can act on transient
structure by suffixing them with an exclamation `!` mark.

```clj
(println (persistent! (conj! (transient []) 1)))
```

# Matching

```clj
(let [[a b & rest :as full] [1 2 3 4]]
    (println a)
    (println b)
    (println rest)
    (println full))
; 1
; 2
; (3 4)
; [1 2 3 4]
```

```clj
(let [[a] []]
    (println a))
nil
```

```clj
(let [{:keys [name]} {:name 1 :other 2}]
  (println name)))
1
```

Map destructuring by providing default values:

```clj
(let [{:keys [name missing] :or {name 3 missing 2}} {:name 1 :other 2}]
  (println name)
  (println missing)))
```

# Threading macros

```clj
; Continuously send output as the last argument.
; `->` same, but send as first argument
; `as-> item alias` use `alias` to refer to the previous item.
(->> "a\nb\n  \n\nc"
     str/split-lines
     (map str/trim)
     (remove empty?))
```

# Threading

```clj
(pmap inc [1 2 3]) ; Map in parallel.
```

```clj
; Delayed execution.
; Execution function on first access (further access will return cached value).
(def a (delay (Thread/sleep 1000) (rand-int 1000)))
(Thread/sleep 1000)
(println @a) ; Returns after 1 second.
(println @a) ; Return same value immediately
```

```clj
; Background execution.
(def a (future (Thread/sleep 1000) (rand-int 1000)))
(Thread/sleep 1000)
(println @a) ; Returns immediatly as 1s already passed.
(println @a) ; Return same value immediately
```

```clj
; Eventual value.
(def a (promise))
(println (realized? a)) ; False no value yet.
(deliver a 42)
(println (realized? a)) ; True.
(println @a) ; 42
(deliver a 43) ; Further deliveries are no ops.
(println @a) ; 42
```

```clj
; Atoms.
(def a (atom 42))
(reset! a 12) ; 12
(swap! a dec) ; 11
(println @a) ; 11
; Note: compare and set does not implement value semantics (has to
; be the same ref.
(compare-and-set! a 12 15) ; false (no change as previous value is not 12)
(compare-and-set! a 11 15) ; true (changed).
; Execution function on changes to atom.
(add-watch a :watch-key (fn [key ref old new] (println old)))
; Only allow to set to values below 20.
(set-validator! a #(< % 20))
(reset! a 21) ; Exception.
```

```clj
In transaction deref (@ref) will read value of ref as it was at the start of transaction. Changes to ref will not retry read only transaction (stale read). To make sure newest val is read use (ensure ref).
```

# Metadata

Attach arbitrary metadata to a `def` variable:

```clj
(def a ^{:my-metadata 1} [1 2 3])

(do
  (println a)
  (println (meta a))

[1 2 3]
{:my-metadata 1}
```

Short hand to attach a boolean equal to `true`:

```clj
(def a ^:my-metadata [1 2 3])

(do
  (println a)
  (println (meta a))

[1 2 3]
{:my-metadata true}
```

# Strings

```clj
(= -2 (parse-long "-2"))
(= \P (char 80))
(= 80 (int \P))
```

```clj
(:require [clojure.string :as str]
(= "a,b" (str/join "," ["a" "b"])
(= "a" (str/trim " a  "))
(= {:x "123", :y "65"}
   (zipmap [:x :y] (rest (re-matches #"^x=(\d+),y=(\d+)$" "x=123,y=65"))))
```

# Math

```clj
(= (quot 10 3) 3) ; Floor(10 / 3).
(defn ceil-div [a b])
    (+ (quot a b) (if (zero? (mod a b)) 0 1))
(= (mod 10 3) 1)
(= 10/3 (/ 10 3)) ; By default uses fractions.
1234567N ; BigInt
```

# Input/Output

```clj
(println (slurp *in*)) ; Read stdin into a variable.
(println (slurp "file.in")) ; Read file into a variable.
; Read file line by line.
(with-open [rdr (clojure.java.io/reader "file.in")]
    (doseq [line (line-seq rdr)]
      (println line))))
```

# Libraries

## Double linked list

```clj
(:require [clojure.data.finger-tree :as finger])

; Operating on left side.
(= (first (finger/double-list 1 2 3)) 1)
(= (finger/conjl (finger/double-list 1 2 3) 0) [0 1 2 3])
(= (rest (finger/double-list 1 2 3)) [2 3])

; Operating on right side.
(= (peek (finger/double-list 1 2 3)) 3)
(= (conj (finger/double-list 1 2 3) 4) [1 2 3 4])
(= (into (finger/double-list 1 2 3) [4 5]) [1 2 3 4 5])
(= (pop (finger/double-list 1 2 3)) [1 2])

; Ensure input did not change to sequence (e.g. `drop`).
(assert (instance? clojure.data.finger_tree.DoubleList input))]
```

## Priority queue

```clj
(:require [clojure.data.priority-map :refer [priority-map]])

(priority-map state1 dist1 state2 dist2 state3 dist3)
(= (peek (priority-map :a 5 :b 2 :c 9)) [:b 2])
(= (pop (priority-map :a 5 :b 2 :c 9)) {:a 5 :c 9})
```

## Permutations

```clj
(:require [clojure.math.combinatorics :as combo])
(= (combo/permutations (range 2)) [[0 1] [1 0]])
(= (combo/subsets (range 2)) [[] [0] [1] [0 1]])
```

# Project Management

```sh
lein new app d01_1
rm -v -rf d01_1/{doc/,CHANGELOG.md,LICENSE,README.md,resources/,.gitignore,.hgignore}
```

```clj
; Increase stack size to 1Gb in Lein projectclj.
:jvm-opts ["-Xss1g"]
```
