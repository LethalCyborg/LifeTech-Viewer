//! Firestorm's catalogue of group abilities (indra/newview/skins/default/xui/
//! en/role_actions.xml, read by LLGroupMgr::parseRoleActions, originally
//! LGPL 2.1), generated: the sets in their order, each ability with its
//! power bit (roles_constants.h) and its texts (`tr!` keys).

/// One ability: its power bit, short and long texts.
pub struct Ability {
    pub bit: u32,
    pub name: &'static str,
    pub help: &'static str,
}

/// A set of abilities, its icon and its texts.
pub struct AbilitySet {
    pub icon: &'static str,
    pub name: &'static str,
    pub help: &'static str,
    pub abilities: Vec<Ability>,
}

pub fn catalogue() -> Vec<AbilitySet> {
    vec![
        AbilitySet {
            icon: "users",
            name: lifetech_i18n::tr!("group-abilities-membership"),
            help: lifetech_i18n::tr!("group-abilities-membership-help"),
            abilities: vec![
                Ability {
                    bit: 1,
                    name: lifetech_i18n::tr!("group-ability-member-invite"),
                    help: lifetech_i18n::tr!("group-ability-member-invite-help"),
                },
                Ability {
                    bit: 2,
                    name: lifetech_i18n::tr!("group-ability-member-eject"),
                    help: lifetech_i18n::tr!("group-ability-member-eject-help"),
                },
                Ability {
                    bit: 51,
                    name: lifetech_i18n::tr!("group-ability-allow-ban"),
                    help: lifetech_i18n::tr!("group-ability-allow-ban-help"),
                },
                Ability {
                    bit: 3,
                    name: lifetech_i18n::tr!("group-ability-member-options"),
                    help: lifetech_i18n::tr!("group-ability-member-options-help"),
                },
            ],
        },
        AbilitySet {
            icon: "crown",
            name: lifetech_i18n::tr!("group-abilities-roles"),
            help: lifetech_i18n::tr!("group-abilities-roles-help"),
            abilities: vec![
                Ability {
                    bit: 4,
                    name: lifetech_i18n::tr!("group-ability-role-create"),
                    help: lifetech_i18n::tr!("group-ability-role-create-help"),
                },
                Ability {
                    bit: 5,
                    name: lifetech_i18n::tr!("group-ability-role-delete"),
                    help: lifetech_i18n::tr!("group-ability-role-delete-help"),
                },
                Ability {
                    bit: 6,
                    name: lifetech_i18n::tr!("group-ability-role-properties"),
                    help: lifetech_i18n::tr!("group-ability-role-properties-help"),
                },
                Ability {
                    bit: 7,
                    name: lifetech_i18n::tr!("group-ability-role-assign-member-limited"),
                    help: lifetech_i18n::tr!("group-ability-role-assign-member-limited-help"),
                },
                Ability {
                    bit: 8,
                    name: lifetech_i18n::tr!("group-ability-role-assign-member"),
                    help: lifetech_i18n::tr!("group-ability-role-assign-member-help"),
                },
                Ability {
                    bit: 9,
                    name: lifetech_i18n::tr!("group-ability-role-remove-member"),
                    help: lifetech_i18n::tr!("group-ability-role-remove-member-help"),
                },
                Ability {
                    bit: 10,
                    name: lifetech_i18n::tr!("group-ability-role-change-actions"),
                    help: lifetech_i18n::tr!("group-ability-role-change-actions-help"),
                },
            ],
        },
        AbilitySet {
            icon: "id-filled",
            name: lifetech_i18n::tr!("group-abilities-group-identity"),
            help: lifetech_i18n::tr!("group-abilities-group-identity-help"),
            abilities: vec![Ability {
                bit: 11,
                name: lifetech_i18n::tr!("group-ability-group-change-identity"),
                help: lifetech_i18n::tr!("group-ability-group-change-identity-help"),
            }],
        },
        AbilitySet {
            icon: "map-2",
            name: lifetech_i18n::tr!("group-abilities-parcel-management"),
            help: lifetech_i18n::tr!("group-abilities-parcel-management-help"),
            abilities: vec![
                Ability {
                    bit: 12,
                    name: lifetech_i18n::tr!("group-ability-land-deed"),
                    help: lifetech_i18n::tr!("group-ability-land-deed-help"),
                },
                Ability {
                    bit: 13,
                    name: lifetech_i18n::tr!("group-ability-land-release"),
                    help: lifetech_i18n::tr!("group-ability-land-release-help"),
                },
                Ability {
                    bit: 14,
                    name: lifetech_i18n::tr!("group-ability-land-set-sale-info"),
                    help: lifetech_i18n::tr!("group-ability-land-set-sale-info-help"),
                },
                Ability {
                    bit: 15,
                    name: lifetech_i18n::tr!("group-ability-land-divide-join"),
                    help: lifetech_i18n::tr!("group-ability-land-divide-join-help"),
                },
            ],
        },
        AbilitySet {
            icon: "map-pin-filled",
            name: lifetech_i18n::tr!("group-abilities-parcel-identity"),
            help: lifetech_i18n::tr!("group-abilities-parcel-identity-help"),
            abilities: vec![
                Ability {
                    bit: 17,
                    name: lifetech_i18n::tr!("group-ability-land-find-places"),
                    help: lifetech_i18n::tr!("group-ability-land-find-places-help"),
                },
                Ability {
                    bit: 18,
                    name: lifetech_i18n::tr!("group-ability-land-change-identity"),
                    help: lifetech_i18n::tr!("group-ability-land-change-identity-help"),
                },
                Ability {
                    bit: 19,
                    name: lifetech_i18n::tr!("group-ability-land-set-landing-point"),
                    help: lifetech_i18n::tr!("group-ability-land-set-landing-point-help"),
                },
            ],
        },
        AbilitySet {
            icon: "adjustments-filled",
            name: lifetech_i18n::tr!("group-abilities-parcel-settings"),
            help: lifetech_i18n::tr!("group-abilities-parcel-settings-help"),
            abilities: vec![
                Ability {
                    bit: 20,
                    name: lifetech_i18n::tr!("group-ability-land-change-media"),
                    help: lifetech_i18n::tr!("group-ability-land-change-media-help"),
                },
                Ability {
                    bit: 21,
                    name: lifetech_i18n::tr!("group-ability-land-edit"),
                    help: lifetech_i18n::tr!("group-ability-land-edit-help"),
                },
                Ability {
                    bit: 22,
                    name: lifetech_i18n::tr!("group-ability-land-options"),
                    help: lifetech_i18n::tr!("group-ability-land-options-help"),
                },
                Ability {
                    bit: 46,
                    name: lifetech_i18n::tr!("group-ability-land-change-environment"),
                    help: lifetech_i18n::tr!("group-ability-land-change-environment-help"),
                },
            ],
        },
        AbilitySet {
            icon: "bolt-filled",
            name: lifetech_i18n::tr!("group-abilities-parcel-powers"),
            help: lifetech_i18n::tr!("group-abilities-parcel-powers-help"),
            abilities: vec![
                Ability {
                    bit: 23,
                    name: lifetech_i18n::tr!("group-ability-land-allow-edit-land"),
                    help: lifetech_i18n::tr!("group-ability-land-allow-edit-land-help"),
                },
                Ability {
                    bit: 24,
                    name: lifetech_i18n::tr!("group-ability-land-allow-fly"),
                    help: lifetech_i18n::tr!("group-ability-land-allow-fly-help"),
                },
                Ability {
                    bit: 25,
                    name: lifetech_i18n::tr!("group-ability-land-allow-create"),
                    help: lifetech_i18n::tr!("group-ability-land-allow-create-help"),
                },
                Ability {
                    bit: 26,
                    name: lifetech_i18n::tr!("group-ability-land-allow-direct-teleport"),
                    help: lifetech_i18n::tr!("group-ability-land-allow-direct-teleport-help"),
                },
                Ability {
                    bit: 28,
                    name: lifetech_i18n::tr!("group-ability-land-allow-set-home"),
                    help: lifetech_i18n::tr!("group-ability-land-allow-set-home-help"),
                },
                Ability {
                    bit: 41,
                    name: lifetech_i18n::tr!("group-ability-land-allow-host-event"),
                    help: lifetech_i18n::tr!("group-ability-land-allow-host-event-help"),
                },
            ],
        },
        AbilitySet {
            icon: "lock-filled",
            name: lifetech_i18n::tr!("group-abilities-parcel-access"),
            help: lifetech_i18n::tr!("group-abilities-parcel-access-help"),
            abilities: vec![
                Ability {
                    bit: 29,
                    name: lifetech_i18n::tr!("group-ability-land-manage-allowed"),
                    help: lifetech_i18n::tr!("group-ability-land-manage-allowed-help"),
                },
                Ability {
                    bit: 30,
                    name: lifetech_i18n::tr!("group-ability-land-manage-banned"),
                    help: lifetech_i18n::tr!("group-ability-land-manage-banned-help"),
                },
                Ability {
                    bit: 31,
                    name: lifetech_i18n::tr!("group-ability-land-manage-passes"),
                    help: lifetech_i18n::tr!("group-ability-land-manage-passes-help"),
                },
                Ability {
                    bit: 32,
                    name: lifetech_i18n::tr!("group-ability-land-admin"),
                    help: lifetech_i18n::tr!("group-ability-land-admin-help"),
                },
            ],
        },
        AbilitySet {
            icon: "photo-filled",
            name: lifetech_i18n::tr!("group-abilities-parcel-content"),
            help: lifetech_i18n::tr!("group-abilities-parcel-content-help"),
            abilities: vec![
                Ability {
                    bit: 48,
                    name: lifetech_i18n::tr!("group-ability-land-return-group-owned"),
                    help: lifetech_i18n::tr!("group-ability-land-return-group-owned-help"),
                },
                Ability {
                    bit: 33,
                    name: lifetech_i18n::tr!("group-ability-land-return-group-set"),
                    help: lifetech_i18n::tr!("group-ability-land-return-group-set-help"),
                },
                Ability {
                    bit: 34,
                    name: lifetech_i18n::tr!("group-ability-land-return-non-group"),
                    help: lifetech_i18n::tr!("group-ability-land-return-non-group-help"),
                },
                Ability {
                    bit: 35,
                    name: lifetech_i18n::tr!("group-ability-land-gardening"),
                    help: lifetech_i18n::tr!("group-ability-land-gardening-help"),
                },
            ],
        },
        AbilitySet {
            icon: "package",
            name: lifetech_i18n::tr!("group-abilities-object-management"),
            help: lifetech_i18n::tr!("group-abilities-object-management-help"),
            abilities: vec![
                Ability {
                    bit: 36,
                    name: lifetech_i18n::tr!("group-ability-object-deed"),
                    help: lifetech_i18n::tr!("group-ability-object-deed-help"),
                },
                Ability {
                    bit: 38,
                    name: lifetech_i18n::tr!("group-ability-object-manipulate"),
                    help: lifetech_i18n::tr!("group-ability-object-manipulate-help"),
                },
                Ability {
                    bit: 39,
                    name: lifetech_i18n::tr!("group-ability-object-set-sale"),
                    help: lifetech_i18n::tr!("group-ability-object-set-sale-help"),
                },
            ],
        },
        AbilitySet {
            icon: "coin-filled",
            name: lifetech_i18n::tr!("group-abilities-accounting"),
            help: lifetech_i18n::tr!("group-abilities-accounting-help"),
            abilities: vec![Ability {
                bit: 40,
                name: lifetech_i18n::tr!("group-ability-accounting-accountable"),
                help: lifetech_i18n::tr!("group-ability-accounting-accountable-help"),
            }],
        },
        AbilitySet {
            icon: "speakerphone",
            name: lifetech_i18n::tr!("group-abilities-notices"),
            help: lifetech_i18n::tr!("group-abilities-notices-help"),
            abilities: vec![
                Ability {
                    bit: 42,
                    name: lifetech_i18n::tr!("group-ability-notices-send"),
                    help: lifetech_i18n::tr!("group-ability-notices-send-help"),
                },
                Ability {
                    bit: 43,
                    name: lifetech_i18n::tr!("group-ability-notices-receive"),
                    help: lifetech_i18n::tr!("group-ability-notices-receive-help"),
                },
            ],
        },
        AbilitySet {
            icon: "messages-filled",
            name: lifetech_i18n::tr!("group-abilities-chat"),
            help: lifetech_i18n::tr!("group-abilities-chat-help"),
            abilities: vec![
                Ability {
                    bit: 16,
                    name: lifetech_i18n::tr!("group-ability-join-group-chat"),
                    help: lifetech_i18n::tr!("group-ability-join-group-chat-help"),
                },
                Ability {
                    bit: 27,
                    name: lifetech_i18n::tr!("group-ability-join-voice-chat"),
                    help: lifetech_i18n::tr!("group-ability-join-voice-chat-help"),
                },
                Ability {
                    bit: 37,
                    name: lifetech_i18n::tr!("group-ability-moderate-group-chat"),
                    help: lifetech_i18n::tr!("group-ability-moderate-group-chat-help"),
                },
            ],
        },
        AbilitySet {
            icon: "key-filled",
            name: lifetech_i18n::tr!("group-abilities-experience-tools-experience"),
            help: lifetech_i18n::tr!("group-abilities-experience-tools-experience-help"),
            abilities: vec![
                Ability {
                    bit: 49,
                    name: lifetech_i18n::tr!("group-ability-experience-admin"),
                    help: lifetech_i18n::tr!("group-ability-experience-admin-help"),
                },
                Ability {
                    bit: 50,
                    name: lifetech_i18n::tr!("group-ability-experience-contributor"),
                    help: lifetech_i18n::tr!("group-ability-experience-contributor-help"),
                },
            ],
        },
    ]
}
