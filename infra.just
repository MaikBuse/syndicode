# Infrastructure management with Terraform

# Show Terraform plan
[group('terraform')]
plan:
  cd infrastructure/terraform && terraform plan

# Apply Terraform changes
[group('terraform')]
apply:
  cd infrastructure/terraform && terraform apply

# Initialize Terraform
[group('terraform')]
init:
  cd infrastructure/terraform && terraform init

# Format Terraform files
[group('terraform')]
fmt:
  cd infrastructure/terraform && terraform fmt -recursive

# Validate Terraform configuration
[group('terraform')]
validate:
  cd infrastructure/terraform && terraform validate

# Import existing R2 bucket
[group('terraform')]
import-bucket:
  cd infrastructure/terraform && terraform import cloudflare_r2_bucket.assets contact@maikbuse.com/syndicode-assets

# Show Terraform state
[group('terraform')]
show:
  cd infrastructure/terraform && terraform show

# Destroy infrastructure (use with caution)
[group('terraform')]
destroy:
  cd infrastructure/terraform && terraform destroy