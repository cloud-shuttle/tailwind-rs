//! Animation Utilities Module
//!
//! Comprehensive animation utilities for Tailwind-RS:
//! - Basic animations: spin, ping, pulse, bounce
//! - Extended animations: fade, slide, zoom, wobble, shake, flip, heartbeat
//! - Animation builder for complex sequences
//! - Animation presets for common use cases

// Re-export all animation components
pub mod types;
pub mod utilities;

// Re-export all types and utilities for easy access
pub use types::*;
pub use utilities::*;

/// Main animation utilities struct that implements the trait
#[derive(Debug, Clone)]
pub struct AnimationUtilitiesImpl;

impl AnimationUtilitiesImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AnimationUtilitiesImpl {
    fn default() -> Self {
        Self::new()
    }
}

/// Animation configuration for advanced use cases
#[derive(Debug, Clone)]
pub struct AnimationConfig {
    pub default_duration: u32,
    pub default_timing: AnimationTiming,
    pub enable_extended_animations: bool,
    pub enable_presets: bool,
}

impl AnimationConfig {
    /// Create a new animation configuration with defaults
    pub fn new() -> Self {
        Self {
            default_duration: 1000,
            default_timing: AnimationTiming::Ease,
            enable_extended_animations: true,
            enable_presets: true,
        }
    }

    /// Configure default duration
    pub fn with_duration(mut self, duration: u32) -> Self {
        self.default_duration = duration;
        self
    }

    /// Configure default timing function
    pub fn with_timing(mut self, timing: AnimationTiming) -> Self {
        self.default_timing = timing;
        self
    }

    /// Enable or disable extended animations
    pub fn extended_animations(mut self, enabled: bool) -> Self {
        self.enable_extended_animations = enabled;
        self
    }

    /// Enable or disable presets
    pub fn presets(mut self, enabled: bool) -> Self {
        self.enable_presets = enabled;
        self
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Animation registry for managing available animations
#[derive(Debug, Clone)]
pub struct AnimationRegistry {
    config: AnimationConfig,
    available_animations: Vec<Animation>,
}

impl AnimationRegistry {
    /// Create a new animation registry
    pub fn new(config: AnimationConfig) -> Self {
        let mut available_animations = Animation::all_values()
            .into_iter()
            .filter(|a| !matches!(a, Animation::None))
            .collect::<Vec<_>>();

        // Filter out extended animations if disabled
        if !config.enable_extended_animations {
            available_animations.retain(|a| a.is_basic());
        }

        Self {
            config,
            available_animations,
        }
    }

    /// Get all available animations
    pub fn available_animations(&self) -> &[Animation] {
        &self.available_animations
    }

    /// Check if an animation is available
    pub fn is_available(&self, animation: &Animation) -> bool {
        self.available_animations.contains(animation)
    }

    /// Get animations by category
    pub fn animations_by_category(&self, category: AnimationCategory) -> Vec<Animation> {
        self.available_animations.iter()
            .filter(|a| a.category() == category)
            .cloned()
            .collect()
    }

    /// Get basic animations only
    pub fn basic_animations(&self) -> Vec<Animation> {
        self.available_animations.iter()
            .filter(|a| a.is_basic())
            .cloned()
            .collect()
    }

    /// Get extended animations only
    pub fn extended_animations(&self) -> Vec<Animation> {
        self.available_animations.iter()
            .filter(|a| a.is_extended())
            .cloned()
            .collect()
    }

    /// Get infinite animations
    pub fn infinite_animations(&self) -> Vec<Animation> {
        self.available_animations.iter()
            .filter(|a| a.is_infinite())
            .cloned()
            .collect()
    }

    /// Get the configuration
    pub fn config(&self) -> &AnimationConfig {
        &self.config
    }
}

impl Default for AnimationRegistry {
    fn default() -> Self {
        Self::new(AnimationConfig::default())
    }
}

/// Animation performance utilities
pub mod performance {
    use super::*;

    /// Calculate estimated animation performance cost
    pub fn estimate_performance_cost(animation: &Animation) -> u8 {
        match animation {
            Animation::None => 0,
            Animation::Spin | Animation::Bounce => 1, // Simple transforms
            Animation::Ping | Animation::Pulse => 2, // Opacity changes
            Animation::FadeIn | Animation::FadeOut => 3, // Opacity animations
            Animation::ZoomIn | Animation::ZoomOut => 4, // Scale transforms
            Animation::SlideInLeft | Animation::SlideInRight | Animation::SlideInTop | Animation::SlideInBottom => 5, // Transform + opacity
            Animation::Shake | Animation::Wobble => 6, // Complex transforms
            Animation::Flip => 7, // 3D transforms
            Animation::Heartbeat => 8, // Complex multi-property animation
        }
    }

    /// Check if animation might cause performance issues on mobile
    pub fn may_cause_mobile_performance_issues(animation: &Animation) -> bool {
        estimate_performance_cost(animation) >= 6
    }

    /// Suggest optimized alternatives for performance-critical animations
    pub fn suggest_performance_alternative(animation: &Animation) -> Option<Animation> {
        match animation {
            Animation::Wobble => Some(Animation::Shake),
            Animation::Flip => Some(Animation::ZoomIn),
            Animation::Heartbeat => Some(Animation::Pulse),
            _ => None,
        }
    }

    /// Check if animation uses hardware acceleration
    pub fn uses_hardware_acceleration(animation: &Animation) -> bool {
        match animation {
            Animation::None => false,
            Animation::Spin | Animation::Bounce | Animation::Flip => true, // Uses transform
            Animation::Ping | Animation::Pulse | Animation::FadeIn | Animation::FadeOut => false, // Uses opacity
            Animation::SlideInLeft | Animation::SlideInRight | Animation::SlideInTop | Animation::SlideInBottom => true, // Uses transform
            Animation::ZoomIn | Animation::ZoomOut => true, // Uses transform
            Animation::Shake | Animation::Wobble => true, // Uses transform
            Animation::Heartbeat => true, // Uses transform + opacity
        }
    }
}

/// Animation accessibility utilities
pub mod accessibility {
    use super::*;

    /// Check if animation respects user's motion preferences
    pub fn respects_motion_preferences(animation: &Animation) -> bool {
        // Essential animations that don't respect preferences
        matches!(animation,
            Animation::None |
            Animation::Spin // Loading indicators are essential
        )
    }

    /// Suggest reduced motion alternative
    pub fn reduced_motion_alternative(animation: &Animation) -> Option<Animation> {
        match animation {
            Animation::Bounce => Some(Animation::FadeIn),
            Animation::Wobble | Animation::Shake => Some(Animation::FadeIn),
            Animation::Flip => Some(Animation::FadeIn),
            Animation::Heartbeat => Some(Animation::Pulse),
            _ => None,
        }
    }

    /// Check if animation is essential for functionality
    pub fn is_essential_for_functionality(animation: &Animation) -> bool {
        matches!(animation, Animation::Spin) // Loading spinners are essential
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animation_config_creation() {
        let config = AnimationConfig::new();
        assert_eq!(config.default_duration, 1000);
        assert_eq!(config.default_timing, AnimationTiming::Ease);
        assert!(config.enable_extended_animations);
        assert!(config.enable_presets);
    }

    #[test]
    fn animation_config_chaining() {
        let config = AnimationConfig::new()
            .with_duration(2000)
            .with_timing(AnimationTiming::EaseInOut)
            .extended_animations(false)
            .presets(false);

        assert_eq!(config.default_duration, 2000);
        assert_eq!(config.default_timing, AnimationTiming::EaseInOut);
        assert!(!config.enable_extended_animations);
        assert!(!config.enable_presets);
    }

    #[test]
    fn animation_registry_basic() {
        let registry = AnimationRegistry::default();

        assert!(!registry.available_animations().is_empty());
        assert!(registry.is_available(&Animation::Spin));
        assert!(registry.is_available(&Animation::FadeIn));
    }

    #[test]
    fn animation_registry_filtered() {
        let config = AnimationConfig::new().extended_animations(false);
        let registry = AnimationRegistry::new(config);

        let basic = registry.basic_animations();
        assert!(basic.contains(&Animation::Spin));
        assert!(!basic.contains(&Animation::FadeIn));

        let extended = registry.extended_animations();
        assert!(extended.is_empty());
    }

    #[test]
    fn animation_registry_by_category() {
        let registry = AnimationRegistry::default();

        let infinite = registry.animations_by_category(AnimationCategory::Infinite);
        assert!(infinite.contains(&Animation::Spin));
        assert!(infinite.contains(&Animation::Bounce));

        let fade = registry.animations_by_category(AnimationCategory::Fade);
        assert!(fade.contains(&Animation::FadeIn));
        assert!(fade.contains(&Animation::FadeOut));
    }

    #[test]
    fn performance_utilities() {
        assert_eq!(performance::estimate_performance_cost(&Animation::Spin), 1);
        assert_eq!(performance::estimate_performance_cost(&Animation::Heartbeat), 8);

        assert!(performance::may_cause_mobile_performance_issues(&Animation::Wobble));
        assert!(!performance::may_cause_mobile_performance_issues(&Animation::Spin));

        assert_eq!(performance::suggest_performance_alternative(&Animation::Wobble), Some(Animation::Shake));

        assert!(performance::uses_hardware_acceleration(&Animation::Spin));
        assert!(!performance::uses_hardware_acceleration(&Animation::Pulse));
    }

    #[test]
    fn accessibility_utilities() {
        assert!(accessibility::respects_motion_preferences(&Animation::None));
        assert!(accessibility::respects_motion_preferences(&Animation::Spin)); // Loading indicators are essential
        assert!(!accessibility::respects_motion_preferences(&Animation::Bounce));

        assert_eq!(accessibility::reduced_motion_alternative(&Animation::Bounce), Some(Animation::FadeIn));
        assert_eq!(accessibility::reduced_motion_alternative(&Animation::Spin), None);

        assert!(accessibility::is_essential_for_functionality(&Animation::Spin));
        assert!(!accessibility::is_essential_for_functionality(&Animation::Bounce));
    }
}
