import { supabase } from '../app/supabaseClient';
import { getDoWhizApiBaseUrl } from '../analytics';

export async function loadProviderRuntimeState() {
  const { data: { session } } = await supabase.auth.getSession();
  const accessToken = session?.access_token;

  if (!accessToken) {
    return { runtimeState: null, reason: 'not_authenticated' };
  }

  const response = await fetch(`${getDoWhizApiBaseUrl()}/api/workspace/provider-state`, {
    headers: {
      Authorization: `Bearer ${accessToken}`
    }
  });

  if (response.status === 401) {
    return { runtimeState: null, reason: 'not_authenticated' };
  }

  if (!response.ok) {
    return { runtimeState: null, reason: 'unavailable' };
  }

  const payload = await response.json();
  if (!payload?.runtime) {
    return { runtimeState: null, reason: 'unavailable' };
  }

  return { runtimeState: payload, reason: 'ok' };
}
