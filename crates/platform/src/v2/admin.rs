pub mod errors;
pub mod models;

mod authentication_providers;
mod cbac;
mod enrollments;
mod groups;
mod markings;
mod organizations;
mod roles;
mod users;

pub use authentication_providers::AuthenticationProviders;
pub use cbac::{CbacBanners, CbacMarkingRestrictions};
pub use enrollments::{EnrollmentRoleAssignments, Enrollments, Hosts};
pub use groups::{GroupMembers, GroupMembershipExpirationPolicies, GroupProviderInfo, Groups};
pub use markings::{
    MarkingCategories, MarkingCategoryPermissions, MarkingMembers, MarkingRoleAssignments, Markings,
};
pub use organizations::{OrganizationGuestMembers, OrganizationRoleAssignments, Organizations};
pub use roles::Roles;
pub use users::{GroupMemberships, UserProviderInfo, Users};

use crate::transport::Transport;

#[derive(Debug)]
pub struct Admin<'c> {
    transport: &'c Transport,
}

impl<'c> Admin<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn authentication_providers(&self) -> AuthenticationProviders<'_> {
        AuthenticationProviders::new(self.transport)
    }
    pub fn cbac_banners(&self) -> CbacBanners<'_> {
        CbacBanners::new(self.transport)
    }
    pub fn cbac_marking_restrictions(&self) -> CbacMarkingRestrictions<'_> {
        CbacMarkingRestrictions::new(self.transport)
    }
    pub fn enrollments(&self) -> Enrollments<'_> {
        Enrollments::new(self.transport)
    }
    pub fn enrollment_role_assignments(&self) -> EnrollmentRoleAssignments<'_> {
        EnrollmentRoleAssignments::new(self.transport)
    }
    pub fn hosts(&self) -> Hosts<'_> {
        Hosts::new(self.transport)
    }
    pub fn groups(&self) -> Groups<'_> {
        Groups::new(self.transport)
    }
    pub fn group_members(&self) -> GroupMembers<'_> {
        GroupMembers::new(self.transport)
    }
    pub fn group_membership_expiration_policies(&self) -> GroupMembershipExpirationPolicies<'_> {
        GroupMembershipExpirationPolicies::new(self.transport)
    }
    pub fn group_provider_info(&self) -> GroupProviderInfo<'_> {
        GroupProviderInfo::new(self.transport)
    }
    pub fn markings(&self) -> Markings<'_> {
        Markings::new(self.transport)
    }
    pub fn marking_members(&self) -> MarkingMembers<'_> {
        MarkingMembers::new(self.transport)
    }
    pub fn marking_role_assignments(&self) -> MarkingRoleAssignments<'_> {
        MarkingRoleAssignments::new(self.transport)
    }
    pub fn marking_categories(&self) -> MarkingCategories<'_> {
        MarkingCategories::new(self.transport)
    }
    pub fn marking_category_permissions(&self) -> MarkingCategoryPermissions<'_> {
        MarkingCategoryPermissions::new(self.transport)
    }
    pub fn organizations(&self) -> Organizations<'_> {
        Organizations::new(self.transport)
    }
    pub fn organization_guest_members(&self) -> OrganizationGuestMembers<'_> {
        OrganizationGuestMembers::new(self.transport)
    }
    pub fn organization_role_assignments(&self) -> OrganizationRoleAssignments<'_> {
        OrganizationRoleAssignments::new(self.transport)
    }
    pub fn roles(&self) -> Roles<'_> {
        Roles::new(self.transport)
    }
    pub fn users(&self) -> Users<'_> {
        Users::new(self.transport)
    }
    pub fn user_provider_info(&self) -> UserProviderInfo<'_> {
        UserProviderInfo::new(self.transport)
    }
    pub fn group_memberships(&self) -> GroupMemberships<'_> {
        GroupMemberships::new(self.transport)
    }
}
