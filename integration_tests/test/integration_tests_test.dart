// Copyright 2022 Xayn AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import 'package:integration_tests/integration_tests.dart' show asyncApi, api2;
import 'package:test/test.dart';

Future<void> main() async {
  test('add works', () async {
    expect(await asyncApi.add(19, 20), equals(39));
  });

  test('sub works', () async {
    expect(await asyncApi.sub(100, 20), equals(80));
  });

  test('api2 works', () async {
    expect(await api2.getTheByte(), equals(12));
  });
}
