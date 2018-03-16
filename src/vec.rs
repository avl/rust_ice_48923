use std::ops::Add;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::Sub;
//use std::ops::Mul;


#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec2 {
    pub fn dot(self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
    pub fn abs(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn kmul(self, k: f32) -> Vec2 {
        Vec2 {
            x: self.x * k,
            y: self.y * k,
        }
    }
    pub fn zero() -> Vec2 {
        Vec2 { x: 0f32, y: 0f32 }
    }
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x: x, y: y }
    }
    pub fn deg90(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: -self.x,
        }
    }
    pub fn vector_intersect(self, line: Line) -> Option<Vec2> {
        let v = self;
        let p1 = line.p1;
        let p2 = line.p2;
        //println!("Input: p1: {:?}, p2: {:?}, v: {:?}\n",p1,p2,v);
        // k1*v = p1 + k2*(p2-p1)

        // V*K1 = P1 + K2*(P2-P1)

        // V*K1 - K2*(P2-P1) = P1
        // v.x*K1 - K2*(p2.x-p1.x) = p1.x
        // v.y*K1 - K2*(p2.y-p1.y) = p1.y

        // M = (
        //   v.x    -p2.x+p1.x
        //   v.y    -p2.y+p1.y
        //  )

        // M*K = p1
        // M^-1*M*K = M^-1 * p1
        // K = M^-1 * p1

        let m00 = v.x;
        let m10 = v.y;
        let m01 = p1.x - p2.x;
        let m11 = p1.y - p2.y;
        let a = m00;
        let b = m01;
        let c = m10;
        let d = m11;
        let det = a * d - b * c;
        if det.abs() < 1e-12 {
            println!("Unexpected no intersection");
            return None;
        }
        let idet = 1.0 / det;

        let M = [[idet * d, -idet * b], [-idet * c, idet * a]];

        let K1 = M[0][0] * p1.x + M[0][1] * p1.y;
        //let K2 = M[1][0]*p1.x + M[1][1]*p1.y;
        //println!("K1: {}, K2: {}",K1,K2);
        return Some(Vec2::new(K1 * v.x, K1 * v.y));

        /*
		let det = 1.0;
		let M=nalgebra::Matrix2::new(				
				m00,m01,
				m10,m11			
			);

		let minv=M.inv();
		let P1=nalgebra::Vector2::new([p1.x,p1.y]);
		let K=minv * P1;
		let ipoint=v*K[0];
		ipoint
		*/
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn normalized(&self) -> Vec2 {
        let l = self.length();
        if l.abs() < 1.0e-8f32 {
            Vec2::new(1.0, 0.0)
        } else {
            self.kmul(1.0f32 / l)
        }
    }
}
impl Vec3 {
    pub fn xy(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn abs(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn kmul(self, k: f32) -> Vec3 {
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
    pub fn new21(a: Vec2, z: f32) -> Vec3 {
        Vec3 {
            x: a.x,
            y: a.y,
            z: z,
        }
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalized(&self) -> Vec3 {
        let l = self.length();
        if l.abs() < 1.0e-8f32 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            self.kmul(1.0f32 / l)
        }
    }
}
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        *self = Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    p1: Vec2,
    p2: Vec2,
}
impl Line {
    pub fn assert_eq(self, other: Line) {
        if (self.p1 - other.p1).abs() > 1e-8 {
            panic!("p1 differs: {:?} vs {:?}", self.p1, other.p1);
        }
        if (self.p2 - other.p2).abs() > 1e-8 {
            panic!("p2 differs: {:?} vs {:?}", self.p2, other.p2);
        }
    }
    pub fn new2(x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
        Line {
            p1: Vec2::new(x1, y1),
            p2: Vec2::new(x2, y2),
        }
    }
    pub fn new(a: Vec2, b: Vec2) -> Line {
        Line { p1: a, p2: b }
    }
}
