# Data sources
data "cloudflare_zone" "main" {
  name = var.domain_name
}

# R2 Bucket (will be imported)
resource "cloudflare_r2_bucket" "assets" {
  account_id = var.cloudflare_account_id
  name       = var.r2_bucket_name
  location   = "auto"
}

# Custom domain for R2 bucket
resource "cloudflare_record" "assets_cname" {
  zone_id = data.cloudflare_zone.main.id
  name    = var.assets_subdomain
  type    = "CNAME"
  content = "${var.r2_bucket_name}.r2.cloudflarestorage.com"
  proxied = true
  ttl     = 1 # Auto when proxied
}

# Cloudflare Worker for PBF fallback
resource "cloudflare_worker_script" "pbf_fallback" {
  account_id = var.cloudflare_account_id
  name       = "pbf-fallback"
  content    = file("${path.module}/workers/pbf-fallback.js")

  r2_bucket_binding {
    name        = "ASSETS_BUCKET"
    bucket_name = cloudflare_r2_bucket.assets.name
  }
}

# Worker route for assets subdomain
resource "cloudflare_worker_route" "assets_route" {
  zone_id     = data.cloudflare_zone.main.id
  pattern     = "${var.assets_subdomain}.${var.domain_name}/*"
  script_name = cloudflare_worker_script.pbf_fallback.name
}