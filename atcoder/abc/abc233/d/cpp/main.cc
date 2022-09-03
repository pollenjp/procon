#include <bits/stdc++.h>
using namespace std;
using ll = long long;

int main() {
  ll ans = 0;
  ll n, k;
  cin >> n >> k;

  vector<ll> s(n + 1);
  vector<ll> a(n);

  map<ll, ll> m;
  // m[s[0]]++;

  for (int i = 0; i < n; i++) {
    cin >> a[i];
    s[i + 1] = s[i] + a[i];
    m[s[i + 1]]++;
  }
  for (int i = 1; i <= n + 1; i++) {
    ll s_r = k + s[i - 1];
    ans += m[s_r];
    m[s[i]]--;
  }

  cout << ans;
}
