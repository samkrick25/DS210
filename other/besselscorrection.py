import math
def bessel():
    values = [8.2, 9.1, 7.7, 8.6, 6.9, 11.2, 10.1, 9.9, 8.9, 9.2, 7.5, 10.5]
    total = sum(values)
    xbar = total/len(values)
    distances = [(val-xbar)**2.0 for val in values]
    sum_of_squares = sum(distances)
    s = math.sqrt(sum_of_squares/(len(values)-1)) 
    print(f"s = {s}, xbar = {xbar}")

if __name__ == '__main__':
    bessel()