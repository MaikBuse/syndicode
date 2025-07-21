terraform {
  required_version = ">= 1.5"
  
  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
  }

  cloud {
    organization = "syndicode"
    workspaces {
      name = "syndicode-infrastructure"
    }
  }
}

provider "cloudflare" {
  # API token will be set via TF_VAR_cloudflare_api_token
}