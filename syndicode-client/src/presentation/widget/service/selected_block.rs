#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectedBlockRegister {
    #[default]
    UserName,
    UserPassword,
    Email,
    CorporationName,
}

impl SelectedBlockRegister {
    pub fn advance(&mut self) {
        *self = match self {
            SelectedBlockRegister::UserName => Self::UserPassword,
            SelectedBlockRegister::UserPassword => Self::Email,
            SelectedBlockRegister::Email => Self::CorporationName,
            SelectedBlockRegister::CorporationName => Self::UserName,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            SelectedBlockRegister::UserName => Self::CorporationName,
            SelectedBlockRegister::UserPassword => Self::UserName,
            SelectedBlockRegister::Email => Self::UserPassword,
            SelectedBlockRegister::CorporationName => Self::Email,
        };
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectedBlockVerify {
    #[default]
    UserName,
    Code,
}

impl SelectedBlockVerify {
    pub fn advance(&mut self) {
        *self = match self {
            SelectedBlockVerify::UserName => Self::Code,
            SelectedBlockVerify::Code => Self::UserName,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            SelectedBlockVerify::UserName => Self::Code,
            SelectedBlockVerify::Code => Self::UserName,
        };
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectedBlockResend {
    #[default]
    UserName,
}

impl SelectedBlockResend {
    pub fn advance(&mut self) {
        *self = match self {
            SelectedBlockResend::UserName => Self::UserName,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            SelectedBlockResend::UserName => Self::UserName,
        };
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectedBlockLogin {
    #[default]
    UserName,
    UserPassword,
}

impl SelectedBlockLogin {
    pub fn advance(&mut self) {
        *self = match self {
            SelectedBlockLogin::UserName => Self::UserPassword,
            SelectedBlockLogin::UserPassword => Self::UserName,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            SelectedBlockLogin::UserName => Self::UserPassword,
            SelectedBlockLogin::UserPassword => Self::UserName,
        };
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectedBlockCreateUser {
    #[default]
    UserName,
    UserPassword,
    UserEmail,
    UserRole,
    CorporationName,
}

impl SelectedBlockCreateUser {
    pub fn advance(&mut self) {
        *self = match self {
            SelectedBlockCreateUser::UserName => Self::UserPassword,
            SelectedBlockCreateUser::UserPassword => Self::UserEmail,
            SelectedBlockCreateUser::UserEmail => Self::UserRole,
            SelectedBlockCreateUser::UserRole => Self::CorporationName,
            SelectedBlockCreateUser::CorporationName => Self::UserName,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            SelectedBlockCreateUser::UserName => Self::CorporationName,
            SelectedBlockCreateUser::UserPassword => Self::UserName,
            SelectedBlockCreateUser::UserEmail => Self::UserPassword,
            SelectedBlockCreateUser::UserRole => Self::UserEmail,
            SelectedBlockCreateUser::CorporationName => Self::UserRole,
        };
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectedBlockGetUser {
    #[default]
    UserUuid,
}

impl SelectedBlockGetUser {
    pub fn advance(&mut self) {
        *self = match self {
            SelectedBlockGetUser::UserUuid => Self::UserUuid,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            SelectedBlockGetUser::UserUuid => Self::UserUuid,
        };
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectedBlockDeleteUser {
    #[default]
    UserUuid,
}

impl SelectedBlockDeleteUser {
    pub fn advance(&mut self) {
        *self = match self {
            SelectedBlockDeleteUser::UserUuid => Self::UserUuid,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            SelectedBlockDeleteUser::UserUuid => Self::UserUuid,
        };
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectedBlockQueryBusinessListings {
    #[default]
    MinAskingPrice,
    MaxAskingPrice,
    SellerCorporationUuid,
    MarketUuid,
    MinOperationalExpenses,
    MaxOperationalExpenses,
    SortBy,
    SortDirection,
    Limit,
    Offset,
}

impl SelectedBlockQueryBusinessListings {
    pub fn advance(&mut self) {
        *self = match self {
            SelectedBlockQueryBusinessListings::MinAskingPrice => Self::MaxAskingPrice,
            SelectedBlockQueryBusinessListings::MaxAskingPrice => Self::SellerCorporationUuid,
            SelectedBlockQueryBusinessListings::SellerCorporationUuid => Self::MarketUuid,
            SelectedBlockQueryBusinessListings::MarketUuid => Self::MinOperationalExpenses,
            SelectedBlockQueryBusinessListings::MinOperationalExpenses => {
                Self::MaxOperationalExpenses
            }
            SelectedBlockQueryBusinessListings::MaxOperationalExpenses => Self::SortBy,
            SelectedBlockQueryBusinessListings::SortBy => Self::SortDirection,
            SelectedBlockQueryBusinessListings::SortDirection => Self::Limit,
            SelectedBlockQueryBusinessListings::Limit => Self::Offset,
            SelectedBlockQueryBusinessListings::Offset => Self::MinAskingPrice,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            SelectedBlockQueryBusinessListings::MinAskingPrice => Self::Offset,
            SelectedBlockQueryBusinessListings::MaxAskingPrice => Self::MinAskingPrice,
            SelectedBlockQueryBusinessListings::SellerCorporationUuid => Self::MaxAskingPrice,
            SelectedBlockQueryBusinessListings::MarketUuid => Self::SellerCorporationUuid,
            SelectedBlockQueryBusinessListings::MinOperationalExpenses => Self::MarketUuid,
            SelectedBlockQueryBusinessListings::MaxOperationalExpenses => {
                Self::MinOperationalExpenses
            }
            SelectedBlockQueryBusinessListings::SortBy => Self::MaxOperationalExpenses,
            SelectedBlockQueryBusinessListings::SortDirection => Self::SortBy,
            SelectedBlockQueryBusinessListings::Limit => Self::SortDirection,
            SelectedBlockQueryBusinessListings::Offset => Self::Limit,
        };
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectedBlockAcquireBusinessListing {
    #[default]
    BusinessListingUuid,
}

impl SelectedBlockAcquireBusinessListing {
    pub fn advance(&mut self) {
        *self = match self {
            SelectedBlockAcquireBusinessListing::BusinessListingUuid => Self::BusinessListingUuid,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            SelectedBlockAcquireBusinessListing::BusinessListingUuid => Self::BusinessListingUuid,
        };
    }
}
