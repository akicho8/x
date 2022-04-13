require "matrix"

a = Vector[2, 5]
b = Vector[6, 3]

# a から b に投射したときの原点から接点までの長さは？
# お互いの成分同士を掛け算して足ばよいのだけど、先に相手つまり b を正規化しておく
b = b.normalize                     # => Vector[0.8944271909999159, 0.4472135954999579]
len = a.map2(b) {|a, b| a * b }.sum # => 4.024922359499621

# a から b に投射したときの接点sは？
# 上で求めた長さは b の線の上にあるので (正規化したb) * 長さ で設定が求まる
s = b * len                         # => Vector[3.5999999999999996, 1.7999999999999998]

# 接点sからaへのベクトルは？
a - s                          # => Vector[-1.5999999999999996, 3.2]

# その長さは？
(a - s).r                      # => 3.5777087639996634
