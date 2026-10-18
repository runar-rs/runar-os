/// Score the Scheduler uses to determine, which process should be executed next.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchedulerScore(usize);

impl core::ops::Add for SchedulerScore {
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			0: self.0 + rhs.0
		}
	}	
}

impl core::ops::AddAssign for SchedulerScore {
	#[inline(always)]
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
	}
}

impl core::ops::Mul for SchedulerScore {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self::Output {
		Self {
			0: self.0 * rhs.0
		}
	}
}

impl core::ops::MulAssign for SchedulerScore {
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Self) {
		self.0 *= rhs.0;
	}
}

impl core::ops::Sub for SchedulerScore {
	type Output = Self;
	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			0: self.0 - rhs.0
		}
	}
}

impl core::ops::SubAssign for SchedulerScore {
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Self) {
		self.0 -= rhs.0;
	}
}
pub struct Scheduler {
    
}

impl Scheduler {
	/// Creates an empty scheduler.
	pub const fn new() -> Self {
		Self {}
	}
}