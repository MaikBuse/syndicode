output "r2_bucket_name" {
  description = "Name of the R2 bucket"
  value       = cloudflare_r2_bucket.assets.name
}

output "assets_domain" {
  description = "Assets domain URL"
  value       = "https://${var.assets_subdomain}.${var.domain_name}"
}

output "worker_script_name" {
  description = "Name of the Cloudflare Worker script"
  value       = cloudflare_workers_script.pbf_fallback.name
}

output "cloudflare_zone_id" {
  description = "Cloudflare Zone ID"
  value       = data.cloudflare_zone.main.id
}