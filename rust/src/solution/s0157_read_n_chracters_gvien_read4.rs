/**
 * The read4 API is defined as.
 *     fn read4(&self,buf4: &mut [char]) -> i32;
 * You can call it using self.read4(buf4)
 */

struct Solution {}

impl Solution {
    // dummy
    pub fn read4(&self, buf4: &mut [char]) -> i32 {
        0
    }

    pub fn read(&self, buf: &mut [char], n: i32) -> i32 {
        let mut buf4 = ['\0'; 4];
        let mut i = 0;
        while i < n {
            let mut buf4_len = self.read4(&mut buf4);
            if buf4_len == 0 {
                break;
            }
            let mut j = 0;
            while j < buf4_len && i < n {
                buf[i as usize] = buf4[j as usize];
                i += 1;
                j += 1;
            }
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_157() {}
}
