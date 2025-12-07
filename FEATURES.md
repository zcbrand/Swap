# Feature Requests & User Stories

User feedback collected to guide development priorities.

---

## Key Themes

| Theme | Mentions | Priority |
|-------|----------|----------|
| Simplicity / Fast Input | 4/4 | **Critical** |
| Export Data (Excel/Email) | 2/4 | High |
| Auto-detection Intelligence | 2/4 | High |
| Editable User Metrics | 3/4 | High |
| No Subscription Paywalls | 1/4 | Medium |

---

## User Stories

### US-001: Simple Weight + Reps Logging
**Source:** User 4
**Priority:** Critical

> "Being old-school, I like the simplicity of paper and pencil, keep it simple, weight and reps."

**Acceptance Criteria:**
- [ ] Core logging requires only 2 inputs: weight + reps
- [ ] No mandatory fields beyond weight/reps
- [ ] Interface feels as fast as writing on paper

---

### US-002: Smart Field Navigation
**Source:** User 4
**Priority:** High

> "The app should be smart enough to move me through the various fields, and just ask for my input"

**Acceptance Criteria:**
- [ ] Auto-advance to next set after logging
- [ ] Pre-fill weight from previous set
- [ ] Keyboard stays open between sets
- [ ] Minimal taps to complete a workout

---

### US-003: Program Builder (4-6 Week Cycles)
**Source:** User 4
**Priority:** High

> "I'd like to be able to build a 4 to 6 week database with at least six exercises"

**Acceptance Criteria:**
- [ ] Create multi-week training programs
- [ ] Support 6+ exercises per workout
- [ ] Track progression across weeks
- [ ] Copy/duplicate program templates

---

### US-004: Excel Export via Email
**Source:** User 4
**Priority:** High

> "When that particular routine is complete it should be easily exported via email in an Excel format"

**Acceptance Criteria:**
- [ ] Export workout history to CSV/Excel
- [ ] Share via email, AirDrop, Files app
- [ ] Include all sets, weights, reps, dates
- [ ] Export individual workouts or date ranges

---

### US-005: Auto-Rest Detection
**Source:** User 3
**Priority:** Medium

> "I'd like a feature that asks, 'are you resting?' for when I forget to hit pause while on a ruck march."

**Acceptance Criteria:**
- [ ] Detect when user hasn't logged for X seconds
- [ ] Prompt: "Are you resting?"
- [ ] Auto-pause option when phone stationary
- [ ] Don't be intrusive during active sets

---

### US-006: Pace Alerts (Audio Chimes)
**Source:** User 3
**Priority:** Medium

> "I'd like a feature that chimes if you're not at your desired pace, so that I'm not constantly looking at my watch"

**Acceptance Criteria:**
- [ ] Set target pace for cardio workouts
- [ ] Audio/haptic alert when pace drops
- [ ] Customizable alert thresholds
- [ ] Works with screen off

---

### US-007: Improved Auto-Workout Detection
**Source:** User 3
**Priority:** Medium

> "I had already ran 1/4 to 1/2 a mile before the app begins tracking"

**Acceptance Criteria:**
- [ ] Faster workout detection (under 0.1 mile)
- [ ] Retroactively capture missed distance
- [ ] Option to "backfill" when detection is late
- [ ] Learn user patterns for prediction

---

### US-008: Editable Weight/BMI
**Source:** Users 1, 3
**Priority:** High

> "I cannot enter my weight, or BMI, for a more accurate measure of calories burned"

**Acceptance Criteria:**
- [ ] Easy weight logging (home screen access)
- [ ] BMI auto-calculated from height/weight
- [ ] Weight history graph
- [ ] Affects calorie burn calculations

---

### US-009: Saved Meals / Quick Access
**Source:** User 1
**Priority:** Low (nutrition not core focus)

> "I like that it saves my usual meals and it's a quick access to my 'daily journal'"

**Note:** Consider integrating with MyFitnessPal via API rather than building nutrition tracking.

---

### US-010: Barcode Scanning for Nutrition
**Source:** User 1
**Priority:** Low (nutrition not core focus)

> "I can type in or scan a barcode from the label and it will input the exact amount"

**Note:** Out of scope for MVP. Consider API integration with nutrition databases.

---

## Anti-Patterns to Avoid

From user feedback, explicitly **DO NOT**:

| Anti-Pattern | Source | Reason |
|--------------|--------|--------|
| Exercise tutorial videos | User 4 | "That's what the Internet is for" |
| Unsolicited advice | User 4 | "Stay away from giving advice" |
| Lock basic features behind paywall | User 1 | Lost weight editing after paying |
| Remove reminders after subscription | User 1 | Features disappeared after payment |
| Require customer service for basic edits | User 1 | "I don't like not being able to edit on my own" |
| Complex/cluttered interfaces | User 4 | "Keep it simple so people will want to use it" |
| Slow data entry for strength training | User 2 | "Weightlifting stuff horribly annoying to input" |

---

## Competitive Insights

| App | Strength | Weakness |
|-----|----------|----------|
| **FitNotes** | Best weightlifting input | - |
| **MapMyRun** | Great GPS tracking | No pace scrubbing |
| **Samsung Health** | Pace data scrubbing | - |
| **MyFitnessPal** | Barcode scanning, saved meals | Paywall issues, hard to edit metrics |
| **Apple Fitness** | Auto-workout detection | Slow detection, limited input |

---

## Implementation Priority

### Phase 1 (MVP Enhancement)
1. US-001: Simple Weight + Reps ✅ (already done)
2. US-002: Smart Field Navigation
3. US-004: Excel Export

### Phase 2
4. US-003: Program Builder
5. US-008: Editable Weight/BMI
6. US-005: Auto-Rest Detection

### Phase 3 (Cardio Focus)
7. US-006: Pace Alerts
8. US-007: Improved Auto-Detection

---

## Beta Tester

> "Best of luck I'd like to see the finished product or test your Beta" — User 4

**Action:** Add User 4 to TestFlight beta list when ready.
