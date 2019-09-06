/*
   Copyright 2019 Takahiro Yamashita

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

int return_int(void) {
  int ret = 10;
  return ret;
}

int64_t return_int64_t(void) {
  int64_t ret = 2147483648;
  return  ret;
}

void print_int(int val) {
  printf("c: val=%d\n",val);
}

void print_int64_t(int64_t val) {
  printf("c: val=%"PRId64"\n",val);
}
