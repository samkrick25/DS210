import math
def bessel():
    values = [15.6,16.2,22.5,20.5,16.4,19.4,16.6,17.9,12.7,13.9]
    total = sum(values)
    xbar = total/len(values)
    distances = [(val-xbar)**2.0 for val in values]
    sum_of_squares = sum(distances)
    s = math.sqrt(sum_of_squares/(len(values)-1)) 
    print(f"s = {s}, xbar = {xbar}")

if __name__ == '__main__':
    bessel()