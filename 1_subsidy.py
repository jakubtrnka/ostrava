COIN = 10**8
HALVING_PERIOD = 210_000

subsidy = 50 * COIN
sum = 0
while subsidy > 0:
    sum += HALVING_PERIOD * subsidy
    subsidy >>= 1

# SUBSIDY = 50 * COIN
print(sum / COIN)
