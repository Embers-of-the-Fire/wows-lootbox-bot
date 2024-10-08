use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum CurrencyType {
    /// 银币
    Credits,
    /// 达布隆
    Gold,
    /// 全局经验
    FreeXp,
    /// 精英指挥官经验
    EliteXp,
    /// 研究点
    ParagonXp,
    /// 钢铁
    Steel,
    /// 煤炭
    Coal,
    /// 军团代币
    Molybdenum,
    ///  
    Brass,
    /// 网站筹码
    Saltpeter,
    /// 社区代币
    RecruitmentPoints,
    /// 联邦代币
    Eventum3,
    /// 雄鹰代币
    Eventum4,
    /// 不列颠尼亚代币
    Eventum5,
    /// 缤纷代币
    Eventum6,
    /// 足球代币
    Eventum7,
    /// 夏季代币
    Eventum8,
    /// 航海家代币
    Eventum9,
    /// 新年证书
    Eventum10,
    /// 补给券
    EventumCn,
    /// 志愿者代币
    Santium,
    /// 造船阶段
    Dockyardum1,
    /// 造船阶段
    Dockyardum2,
    /// 蔚蓝档案代币
    Eventum11,
    /// 企业代币
    Eventum12,
    /// 节庆证书
    Eventum13,
    /// 圣诞代币
    Eventum14,
    /// 法国代币
    Eventum1,
    /// 金色代币
    Eventum2,
    /// 战斗代币
    Clientum1,
    ///  
    Clientum2,
    /// 石油
    ClanResource,

    // do not edit following
    /// 高级账号
    WowsPremium,
    /// 船坞
    Slots,
}

impl CurrencyType {
    pub const ALL_QUERY_CURRENCIES: [Self; 32] = [
        Self::Credits,
        Self::Gold,
        Self::FreeXp,
        Self::EliteXp,
        Self::ParagonXp,
        Self::Steel,
        Self::Coal,
        Self::Molybdenum,
        Self::Brass,
        Self::Saltpeter,
        Self::RecruitmentPoints,
        Self::Eventum3,
        Self::Eventum4,
        Self::Eventum5,
        Self::Eventum6,
        Self::Eventum7,
        Self::Eventum8,
        Self::Eventum9,
        Self::Eventum10,
        Self::EventumCn,
        Self::Santium,
        Self::Dockyardum1,
        Self::Dockyardum2,
        Self::Eventum11,
        Self::Eventum12,
        Self::Eventum13,
        Self::Eventum14,
        Self::Eventum1,
        Self::Eventum2,
        Self::Clientum1,
        Self::Clientum2,
        Self::ClanResource,
    ];

    pub const ALL_NON_QUERY_CURRENCIES: [Self; 2] = [Self::WowsPremium, Self::Slots];

    pub fn as_icon_name(&self) -> &'static str {
        match self {
            Self::Credits => "credits",
            Self::Gold => "gold",
            Self::FreeXp => "free-xp",
            Self::EliteXp => "elite-xp",
            Self::ParagonXp => "paragon-xp",
            Self::Steel => "steel",
            Self::Coal => "coal",
            Self::Molybdenum => "molybdenum",
            Self::Brass => "brass",
            Self::Saltpeter => "saltpeter",
            Self::RecruitmentPoints => "recruitment-points",
            Self::Eventum3 => "eventum-3",
            Self::Eventum4 => "eventum-4",
            Self::Eventum5 => "eventum-5",
            Self::Eventum6 => "eventum-6",
            Self::Eventum7 => "eventum-7",
            Self::Eventum8 => "eventum-8",
            Self::Eventum9 => "eventum-9",
            Self::Eventum10 => "eventum-10",
            Self::EventumCn => "eventum-cn",
            Self::Santium => "santium",
            Self::Dockyardum1 => "dockyardum-1",
            Self::Dockyardum2 => "dockyardum-2",
            Self::Eventum11 => "eventum-11",
            Self::Eventum12 => "eventum-12",
            Self::Eventum13 => "eventum-13",
            Self::Eventum14 => "eventum-14",
            Self::Eventum1 => "eventum-1",
            Self::Eventum2 => "eventum-2",
            Self::Clientum1 => "clientum-1",
            Self::Clientum2 => "clientum-2",
            Self::ClanResource => "clan-resource",

            // do not edit following
            CurrencyType::WowsPremium => "wows-premium",
            CurrencyType::Slots => "slots",
        }
    }

    pub fn as_name_string(&self, lang: &str) -> &'static str {
        match lang {
            "zh-sg" => match self {
                Self::Credits => "银币",
                Self::Gold => "达布隆",
                Self::FreeXp => "全局经验",
                Self::EliteXp => "精英指挥官经验",
                Self::ParagonXp => "研究点",
                Self::Steel => "钢铁",
                Self::Coal => "煤炭",
                Self::Molybdenum => "军团代币",
                Self::Brass => " ",
                Self::Saltpeter => "网站筹码",
                Self::RecruitmentPoints => "社区代币",
                Self::Eventum3 => "联邦代币",
                Self::Eventum4 => "雄鹰代币",
                Self::Eventum5 => "不列颠尼亚代币",
                Self::Eventum6 => "缤纷代币",
                Self::Eventum7 => "足球代币",
                Self::Eventum8 => "夏季代币",
                Self::Eventum9 => "航海家代币",
                Self::Eventum10 => "新年证书",
                Self::EventumCn => "补给券",
                Self::Santium => "志愿者代币",
                Self::Dockyardum1 => "造船阶段",
                Self::Dockyardum2 => "造船阶段",
                Self::Eventum11 => "蔚蓝档案代币",
                Self::Eventum12 => "企业代币",
                Self::Eventum13 => "节庆证书",
                Self::Eventum14 => "圣诞代币",
                Self::Eventum1 => "法国代币",
                Self::Eventum2 => "金色代币",
                Self::Clientum1 => "战斗代币",
                Self::Clientum2 => " ",
                Self::ClanResource => "石油",
                Self::WowsPremium => "高级账号",
                Self::Slots => "船坞",
            },
            "en" => match self {
                Self::Credits => "Credits",
                Self::Gold => "Doubloons",
                Self::FreeXp => "Free XP",
                Self::EliteXp => "Elite Commander XP",
                Self::ParagonXp => "Research Points",
                Self::Steel => "Steel",
                Self::Coal => "Coal",
                Self::Molybdenum => "Clan Tokens",
                Self::Brass => " ",
                Self::Saltpeter => "Portal Chips",
                Self::RecruitmentPoints => "Community Tokens",
                Self::Eventum3 => "Commonwealth Tokens",
                Self::Eventum4 => "Independence Tokens",
                Self::Eventum5 => "Britannia Tokens",
                Self::Eventum6 => "Colorful Tokens",
                Self::Eventum7 => "Football Tokens",
                Self::Eventum8 => "Summer Tokens",
                Self::Eventum9 => "Navigator Tokens",
                Self::Eventum10 => "New Year Certificates",
                Self::EventumCn => "Supply Vouchers",
                Self::Santium => "Volunteer Tokens",
                Self::Dockyardum1 => "Shipbuilding phases",
                Self::Dockyardum2 => "Shipbuilding phases",
                Self::Eventum11 => "Blue Archive Tokens",
                Self::Eventum12 => "Enterprise Tokens",
                Self::Eventum13 => "Festive Certificates",
                Self::Eventum14 => "Christmas Tokens",
                Self::Eventum1 => "French Tokens",
                Self::Eventum2 => "Golden Tokens",
                Self::Clientum1 => "Battle Tokens",
                Self::Clientum2 => " ",
                Self::ClanResource => "Oil",
                Self::WowsPremium => "Premium Account",
                Self::Slots => "Ship Slot",
            },

            // do not edit following
            _ => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyData {
    pub r#type: CurrencyType,
    pub name: String,
    pub icon: String,
}
