```.sh
lein new app d01_1
rm -v -rf d01_1/{doc/,CHANGELOG.md,LICENSE,README.md,resources/,.gitignore,.hgignore}
```

# Conditions

```.lisp
((cond
   (< -1 0) "negative"
   (> -1 0) "positive"
   :else "zero")))
```

# Functions

```.lisp
(defn func
  ([onearg] (println "one arg variant"))
  ([one two] (println "two arg variant")))
```

Anonymous function with first argument referred as `%` and subsequent ones with `%n`.

```.lisp
(#(println % %2) 1 2))
```

Is equivalent to

```.lisp
((fn [a1 a2] (println a1 a2)) 1 2))
```

# Containers

```.lisp
(conj #{1 2 3} 4)
#{1 3 2 9}
```

```.lisp
(contains? #{1 2 3} 1)
true
```

```.lisp
(empty? [])
true
```

Convert vector into mutable (transient; as a performance optimization if
multiple changes are needed), insert a value into it and convert back
to an immutable version. Note that most operations can act on transient
structure by suffixing them with an exclamation `!` mark.

```.lisp
(println (persistent! (conj! (transient []) 1)))
```

# Matching

```.lisp
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

```.lisp
(let [[a] []]
    (println a))
nil
```

```.lisp
(let [{:keys [name]} {:name 1 :other 2}]
  (println name)))
1
```

Map desctructuring by providing default values:

```.lisp
(let [{:keys [name  missing] :or {name 3 missing 2}} {:name 1 :other 2}]
  (println name)
  (println missing)))
```

# Other

```.lisp
((juxt * +) 2 3) ; [(+ 2 3) (* 2 3)]
6 5
```

```.lisp
; Produce a map where keys are result of calling `:grp` on all items of second
; argument and values are a vector with all agreeing items.
(group-by :grp [{:grp 1} {:grp 1} {:grp 2}])
{1 [{:grp 1} {:grp 1}], 2 [{:grp 2}]}
```

```.lisp
; Update given nested maps (first argument) by a path (second argument) to a
; value (third argument).
(assoc-in {:a {:b 1}} [:a :b] 2)
{:a {:b 2}}
```

```.lisp
; Update given nested maps (first argument) by a path (second argument) to a
; value received by applying a function (third argument)
(println (update-in {:a {:b 1}} [:a :b] #(+ % 1))))
{:a {:b 2}}
```

# Metadata

Attach arbitrary metadata to a `def` variable:

```.lisp
(def a ^{:my-metadata 1} [1 2 3])

(do
  (println a)
  (println (meta a))

[1 2 3]
{:my-metadata 1}
```

Short hand to attach a boolean equal to `true`:

```.lisp
(def a ^:my-metadata [1 2 3])

(do
  (println a)
  (println (meta a))

[1 2 3]
{:my-metadata true}
```

# Files
