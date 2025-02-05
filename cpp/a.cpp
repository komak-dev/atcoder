#include <iostream>
#include <vector>

using namespace std;


int main() {
    int l, d, n;
    cin >> l >> d >> n;

    vector<pair<int, int>> sensor_pos(n);
    vector<int> sensor_range(n);
    for (int i = 0; i < n; i++) {
        cin >> sensor_pos[i].first >> sensor_pos[i].second >> sensor_range[i];
    }


}
