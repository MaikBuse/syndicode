const ASSETS_BASE_URL = 'https://assets.syndicode.dev';

/**
 * Generates the business image URL based on market number, image number, and resolution.
 * 
 * URL Pattern: {assets_base_url}/images/economy/businesses/{market_number}/{image_number}/{image_number}-{resolution}.webp
 * Example: https://assets.syndicode.dev/images/economy/businesses/1/5/5-1920w.webp
 * 
 * @param marketNumber - The market number (from MarketName enum numeric value)
 * @param imageNumber - The image number (1-10)
 * @param resolution - The resolution ('320w', '768w', '1920w', '3840w', 'original')
 * @returns The complete image URL
 */
export function getBusinessImageUrl(
  marketNumber: number,
  imageNumber: number,
  resolution: '320w' | '768w' | '1920w' | '3840w' | 'original'
): string {
  return `${ASSETS_BASE_URL}/images/economy/businesses/${marketNumber}/${imageNumber}/${imageNumber}-${resolution}.webp`;
}

/**
 * Generates a responsive srcSet for business images with all available resolutions.
 * 
 * @param marketNumber - The market number (from MarketName enum numeric value)
 * @param imageNumber - The image number (1-10)
 * @returns A srcSet string for responsive images
 */
export function getBusinessImageSrcSet(
  marketNumber: number,
  imageNumber: number
): string {
  const resolutions: Array<'320w' | '768w' | '1920w' | '3840w'> = ['320w', '768w', '1920w', '3840w'];
  
  return resolutions
    .map(resolution => `${getBusinessImageUrl(marketNumber, imageNumber, resolution)} ${resolution}`)
    .join(', ');
}

/**
 * Generates the default business image URL (1920w resolution).
 * This is typically used as the main src attribute for img elements.
 * 
 * @param marketNumber - The market number (from MarketName enum numeric value)
 * @param imageNumber - The image number (1-10)
 * @returns The default resolution image URL
 */
export function getDefaultBusinessImageUrl(
  marketNumber: number,
  imageNumber: number
): string {
  return getBusinessImageUrl(marketNumber, imageNumber, '1920w');
}