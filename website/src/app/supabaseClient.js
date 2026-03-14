import { createClient } from '@supabase/supabase-js';

export const supabase = createClient(
  'https://resmseutzmwumflevfqw.supabase.co',
  'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InJlc21zZXV0em13dW1mbGV2ZnF3Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3NzAxNTQ1MjIsImV4cCI6MjA4NTczMDUyMn0.-QMndwi4m8nBtjMeS5WbDmrHZSe2l1UFY-UQJCl0Frc'
);
