```.sh
lein new app d01_1
rm -v -rf d01_1/{doc/,CHANGELOG.md,LICENSE,README.md,resources/,.gitignore,.hgignore}
```

# Conditions

```.clj
(if (<= 1 2) "ok" "nok")
(when (<= 1 2) "ok") ; else defaults to nil.
```

```.clj
(cond
  (< -1 0) "negative"
  (> -1 0) "positive"
  :else "zero")
```

```.clj
(case (+ 2 3)
 3 "nok"
 5 "ok"
 "default")
```

# Errors

```.clj
(throw (ex-info "Unknown opcode" {:opcode opcode}))))
(assert (not= 1 2) (str "Unknown opcode: " opcode))
```

# Functions

```.clj
(defn func
  ([onearg] (println "one arg variant"))
  ([one two] (println "two arg variant")))
```

Anonymous function with first argument referred as `%` (or `%1`) and subsequent
ones with `%n`.

```.clj
(#(println % %2) 1 2))
```

Is equivalent to

```.clj
((fn [a1 a2] (println a1 a2)) 1 2))
```

```.clj
(def adder (memoize (fn [x y] (+ x y))))
```

# Containers

```.clj
(conj #{1 2 3} 4)
#{1 3 2 9}
```

```.clj
(contains? #{1 2 3} 1)
true
```

```.clj
(nth [2 9 10 1] 2)
10
```

```.clj
(empty? [])
true
```

Convert vector into mutable (transient; as a performance optimization if
multiple changes are needed), insert a value into it and convert back
to an immutable version. Note that most operations can act on transient
structure by suffixing them with an exclamation `!` mark.

```.clj
(println (persistent! (conj! (transient []) 1)))
```

# Matching

```.clj
(let [[a b & rest :as full] [1 2 3 4]]
    (println a)
    (println b)
    (println rest)
    (println full))
1
2
(3 4)
[1 2 3 4]
```

```.clj
(let [[a] []]
    (println a))
nil
```

```.clj
(let [{:keys [name]} {:name 1 :other 2}]
  (println name)))
1
```

Map desctructuring by providing default values:

```.clj
(let [{:keys [name missing] :or {name 3 missing 2}} {:name 1 :other 2}]
  (println name)
  (println missing)))
```

# Other

```.clj
((juxt * +) 2 3) ; [(+ 2 3) (* 2 3)]
6 5
```

```.clj
; Produce a map where keys are result of calling `:grp` on all items of second
; argument and values are a vector with all agreeing items.
(group-by :grp [{:grp 1} {:grp 1} {:grp 2}])
{1 [{:grp 1} {:grp 1}], 2 [{:grp 2}]}
```

```.clj
; Update given nested maps (first argument) by a path (second argument) to a
; value (third argument).
(assoc-in {:a {:b 1}} [:a :b] 2)
{:a {:b 2}}
```

```.clj
; Update given nested maps (first argument) by a path (second argument) to a
; value received by applying a function (third argument)
(println (update-in {:a {:b 1}} [:a :b] #(+ % 1))))
{:a {:b 2}}
```

# Metadata

Attach arbitrary metadata to a `def` variable:

```.clj
(def a ^{:my-metadata 1} [1 2 3])

(do
  (println a)
  (println (meta a))

[1 2 3]
{:my-metadata 1}
```

Short hand to attach a boolean equal to `true`:

```.clj
(def a ^:my-metadata [1 2 3])

(do
  (println a)
  (println (meta a))

[1 2 3]
{:my-metadata true}
```

# Strings

```.clj
parse-long "-2"
```

# Math

```.clj
(= 3 (quot 10 3)) ; Floor(10 / 3).
(= 10/3 (/ 10 3)) ; By default uses fractions.
1234567N ; BigInt
```

# Libraries

## Double linked list

```.clj
(:require [clojure.data.finger-tree :as finger])

; Operating on left side.
(= 1 (first (finger/double-list 1 2 3)))
(= [0 1 2 3] (finger/conjl (finger/double-list 1 2 3) 0))
(= [2 3] (rest (finger/double-list 1 2 3)))

; Operating on right side.
(= 3 (peek (finger/double-list 1 2 3)))
(= [1 2 3 4] (conj (finger/double-list 1 2 3) 4))
(= [1 2 3 4 5] (into (finger/double-list 1 2 3) [4 5]))
(= [1 2] (pop (finger/double-list 1 2 3)))
```

## Priority queue

```.clj
(:require [clojure.data.priority-map :refer [priority-map]])

(priority-map state1 dist1 state2 dist2 state3 dist3)
(= [:b 2] (peek (priority-map :a 5 :b 2 :c 9)))
(= {:a 5 :c 9} (pop (priority-map :a 5 :b 2 :c 9)))
```
