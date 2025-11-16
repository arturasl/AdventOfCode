(defproject d04_1 "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://example.com/FIXME"
  :license {:name "EPL-2.0 OR GPL-2.0-or-later WITH Classpath-exception-2.0"
            :url "https://www.eclipse.org/legal/epl-2.0/"}
  :dependencies [[org.clojure/clojure "1.12.3"]
                 [org.clojure/math.combinatorics "0.3.0"]
                 [org.clojure/math.numeric-tower "0.1.0"]
                 [org.clojure/data.finger-tree "0.1.0"]
                 [org.clojure/data.priority-map "1.2.0"]]
  :jvm-opts ["-Xss10m"]
  :main ^:skip-aot d16-1.core
  :target-path "target/%s"
  :test-paths ["src/" "test/"]
  :profiles {:uberjar {:aot :all
                       :jvm-opts ["-Dclojure.compiler.direct-linking=true"]}})
