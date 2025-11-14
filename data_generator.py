import random
import csv
from datetime import datetime, timedelta

# Categories and their variations
categories = {
    # Documents
    'reports': ['report', 'quarterly_report', 'annual_report', 'financial_report', 'sales_report', 
                'monthly_report', 'weekly_report', 'status_report', 'progress_report', 'summary_report'],
    'invoices': ['invoice', 'receipt', 'bill', 'statement', 'payment', 'transaction'],
    'contracts': ['contract', 'agreement', 'nda', 'terms', 'proposal', 'quote'],
    'presentations': ['presentation', 'slides', 'deck', 'pitch', 'demo'],
    'meeting_notes': ['meeting_notes', 'notes', 'minutes', 'agenda', 'action_items'],
    'resumes': ['resume', 'cv', 'curriculum_vitae', 'cover_letter', 'application'],
    
    # Media
    'photos': ['photo', 'image', 'picture', 'pic', 'img', 'snapshot', 'shot'],
    'videos': ['video', 'clip', 'footage', 'movie', 'film', 'recording'],
    'music': ['song', 'track', 'audio', 'music', 'recording', 'mix', 'remix'],
    'screenshots': ['screenshot', 'screen_capture', 'screengrab', 'capture'],
    
    # Development
    'code': ['main', 'index', 'app', 'server', 'client', 'utils', 'helpers', 'config'],
    'tests': ['test', 'spec', 'unit_test', 'integration_test', 'e2e_test'],
    'docs': ['readme', 'documentation', 'guide', 'tutorial', 'manual', 'changelog'],
    'data': ['data', 'dataset', 'backup', 'export', 'dump', 'archive'],
    
    # Projects
    'designs': ['design', 'mockup', 'wireframe', 'prototype', 'sketch', 'layout'],
    'drafts': ['draft', 'wip', 'work_in_progress', 'temp', 'temporary'],
    'final': ['final', 'final_version', 'approved', 'published', 'released'],
    
    # Personal
    'taxes': ['tax', 'tax_return', 'w2', '1099', 'deduction', 'expense'],
    'travel': ['itinerary', 'booking', 'reservation', 'ticket', 'hotel', 'flight'],
    'health': ['medical', 'prescription', 'insurance', 'health_record', 'lab_result'],
    'education': ['assignment', 'homework', 'exam', 'quiz', 'syllabus', 'lecture'],
}

# Modifiers and descriptors
years = [str(y) for y in range(2015, 2026)]
months = ['january', 'february', 'march', 'april', 'may', 'june', 
          'july', 'august', 'september', 'october', 'november', 'december']
month_abbrev = ['jan', 'feb', 'mar', 'apr', 'may', 'jun', 'jul', 'aug', 'sep', 'oct', 'nov', 'dec']
quarters = ['q1', 'q2', 'q3', 'q4']
versions = ['v1', 'v2', 'v3', 'final', 'draft', 'revised', 'updated']
companies = ['acme', 'techcorp', 'globex', 'initech', 'hooli', 'pied_piper', 'aperture', 'umbrella']
clients = ['client_a', 'client_b', 'johnson', 'smith', 'williams', 'brown', 'jones']
projects = ['alpha', 'beta', 'gamma', 'delta', 'project_x', 'project_phoenix', 'project_atlas']
priorities = ['urgent', 'high_priority', 'important', 'critical']
statuses = ['pending', 'approved', 'rejected', 'reviewed', 'completed', 'in_progress']

# Semantic descriptors for richer variation
adjectives = ['new', 'old', 'updated', 'revised', 'final', 'draft', 'preliminary', 'detailed', 
              'complete', 'partial', 'full', 'summary', 'comprehensive', 'brief', 'quick',
              'annual', 'monthly', 'weekly', 'daily', 'internal', 'external', 'public', 'private',
              'confidential', 'urgent', 'important', 'archived', 'latest', 'previous']

topics = ['marketing', 'sales', 'finance', 'hr', 'operations', 'strategy', 'product', 'customer',
          'technical', 'legal', 'compliance', 'security', 'research', 'development', 'analysis',
          'budget', 'forecast', 'planning', 'review', 'audit', 'training', 'onboarding']

locations = ['remote', 'office', 'headquarters', 'branch', 'regional', 'global', 'local', 
             'domestic', 'international', 'north_america', 'europe', 'asia', 'east_coast', 
             'west_coast', 'midwest', 'san_francisco', 'new_york', 'london', 'tokyo']

departments = ['engineering', 'marketing', 'sales', 'hr', 'finance', 'operations', 'legal',
               'product', 'design', 'customer_success', 'support', 'research', 'analytics',
               'it', 'admin', 'executive', 'management']

event_types = ['workshop', 'conference', 'seminar', 'training', 'webinar', 'meeting', 'call',
               'standup', 'retrospective', 'planning', 'review', 'brainstorm', 'kickoff',
               'launch', 'demo', 'presentation', 'interview', 'onboarding']

document_types = ['summary', 'overview', 'outline', 'checklist', 'template', 'form', 'worksheet',
                  'guidelines', 'policy', 'procedure', 'handbook', 'reference', 'specs',
                  'requirements', 'roadmap', 'timeline', 'schedule', 'plan', 'strategy']

media_subjects = ['vacation', 'birthday', 'wedding', 'party', 'holiday', 'family', 'friends',
                  'nature', 'landscape', 'portrait', 'sunset', 'beach', 'mountains', 'city',
                  'food', 'pets', 'travel', 'adventure', 'concert', 'festival', 'graduation']

file_actions = ['backup', 'copy', 'export', 'import', 'sync', 'transfer', 'upload', 'download',
                'archive', 'restore', 'migration', 'conversion', 'extraction', 'compilation']

qualities = ['high_quality', 'low_res', 'hd', '4k', 'compressed', 'raw', 'edited', 'unedited',
             'original', 'processed', 'enhanced', 'optimized', 'draft_quality', 'print_ready']

# Common naming patterns
def generate_date_format():
    date = datetime(2020, 1, 1) + timedelta(days=random.randint(0, 1825))
    formats = [
        date.strftime('%Y-%m-%d'),
        date.strftime('%Y%m%d'),
        date.strftime('%d-%m-%Y'),
        date.strftime('%m-%d-%Y'),
        date.strftime('%Y_%m_%d'),
    ]
    return random.choice(formats)

def generate_filename():
    patterns = [
        # Date-based patterns
        lambda cat: f"{random.choice(categories[cat])}_{generate_date_format()}",
        lambda cat: f"{generate_date_format()}_{random.choice(categories[cat])}",
        
        # Versioned patterns
        lambda cat: f"{random.choice(categories[cat])}_{random.choice(versions)}",
        lambda cat: f"{random.choice(categories[cat])}_{random.choice(versions)}_{random.randint(1, 100)}",
        
        # Company/Client patterns
        lambda cat: f"{random.choice(companies)}_{random.choice(categories[cat])}",
        lambda cat: f"{random.choice(categories[cat])}_{random.choice(clients)}",
        
        # Project patterns
        lambda cat: f"{random.choice(projects)}_{random.choice(categories[cat])}",
        
        # Time period patterns
        lambda cat: f"{random.choice(categories[cat])}_{random.choice(years)}",
        lambda cat: f"{random.choice(categories[cat])}_{random.choice(months)}_{random.choice(years)}",
        lambda cat: f"{random.choice(categories[cat])}_{random.choice(quarters)}_{random.choice(years)}",
        lambda cat: f"{random.choice(years)}_{random.choice(quarters)}_{random.choice(categories[cat])}",
        
        # Status patterns
        lambda cat: f"{random.choice(categories[cat])}_{random.choice(statuses)}",
        
        # Numbered patterns
        lambda cat: f"{random.choice(categories[cat])}_{random.randint(1, 9999):04d}",
        lambda cat: f"{random.choice(categories[cat])}_{random.randint(1, 100)}",
        
        # Combined patterns
        lambda cat: f"{random.choice(categories[cat])}_{random.choice(years)}_{random.choice(months)}",
        lambda cat: f"{random.choice(projects)}_{random.choice(categories[cat])}_{random.choice(versions)}",
        lambda cat: f"{random.choice(companies)}_{random.choice(categories[cat])}_{generate_date_format()}",
        lambda cat: f"{random.choice(categories[cat])}_{random.choice(clients)}_{random.choice(years)}",
        
        # Simple patterns
        lambda cat: f"{random.choice(categories[cat])}",
        lambda cat: f"{random.choice(categories[cat])}_{random.choice(['copy', 'backup', 'old', 'new', 'latest'])}",
        
        # Descriptive patterns
        lambda cat: f"{random.choice(['my', 'our', 'team', 'personal'])}_{random.choice(categories[cat])}",
        lambda cat: f"{random.choice(categories[cat])}_{random.choice(['final', 'draft', 'review', 'edited'])}",
    ]
    
    category = random.choice(list(categories.keys()))
    pattern = random.choice(patterns)
    
    return pattern(category)

# Generate dataset
print("Generating 50,000 filenames...")
filenames = set()

while len(filenames) < 50000:
    filename = generate_filename()
    filenames.add(filename)
    
    print (f"Progress: {len(filenames)}/50000", end='\r', flush=True)

# Convert to list and sort for consistency
filenames = sorted(list(filenames))

# Save to CSV
output_file = './datasets/filename_training_dataset_v2.csv'
with open(output_file, 'w', newline='', encoding='utf-8') as f:
    writer = csv.writer(f)
    writer.writerow(['filename'])
    for filename in filenames:
        writer.writerow([filename])

print(f"\nDataset generated successfully!")
print(f"Total filenames: {len(filenames)}")
print(f"Saved to: {output_file}")