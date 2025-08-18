(ns reverse-string)

(require '[clojure.string :as str])

(defn reverse-string [s] ;; <- arglist goes here
  (apply str (for [ i (range (- (count s) 1) -1 -1) ] (get s i)))
)
