variable "cloudflare_account_id" {
  description = "Cloudflare Account ID"
  type        = string
  default     = "contact@maikbuse.com"
}

variable "domain_name" {
  description = "Domain name for assets"
  type        = string
  default     = "syndicode.dev"
}

variable "assets_subdomain" {
  description = "Subdomain for assets"
  type        = string
  default     = "assets"
}

variable "r2_bucket_name" {
  description = "Name of the R2 bucket"
  type        = string
  default     = "syndicode-assets"
}

variable "cloudflare_api_token" {
  description = "Cloudflare API Token"
  type        = string
  sensitive   = true
}