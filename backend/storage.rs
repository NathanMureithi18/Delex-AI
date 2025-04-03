use std::cell::RefCell;
use ic_cdk_macros::*;

thread_local! {
    pub static CONSTITUTION_CHUNKS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

pub fn init_constitution() {
    CONSTITUTION_CHUNKS.with(|chunks| {
        let mut chunks_mut = chunks.borrow_mut();
        
        // Preamble
        chunks_mut.push("LAWS OF KENYA THE CONSTITUTION OF KENYA Revised Edition 2022 Printed and Published by the Government Printer Nairobi".to_string());
        
        // Chapter One
chunks_mut.push("CHAPTER ONE - SOVEREIGNTY OF THE PEOPLE AND SUPREMACY OF THIS CONSTITUTION
Article 1. Sovereignty of the people - Sovereignty belongs to the people of Kenya, and they exercise it directly or through their elected representatives.
Article 2. Supremacy of this Constitution - This Constitution is the supreme law of the land, and any law inconsistent with it is void.
Article 3. Defence of this Constitution - Every person has an obligation to respect, uphold, and defend this Constitution.".to_string());

// Chapter Two
chunks_mut.push("CHAPTER TWO - THE REPUBLIC
Article 4. Declaration of the Republic - Kenya is a sovereign republic with a democratic form of government.
Article 5. Territory of Kenya - The territory of Kenya consists of the lands and waters within the national borders as defined by law.
Article 6. Devolution and access to services - Kenya is a devolved government with a system of governance that allows the sharing of powers and responsibilities with counties.
Article 7. National, official and other languages - Kiswahili and English are the official languages of Kenya, and other languages may also be used.
Article 8. State and religion - There is no state religion, and the state shall ensure freedom of religion and belief.
Article 9. National symbols and national days - The Constitution defines national symbols, including the flag, anthem, and coat of arms, and establishes national holidays.
Article 10. National values and principles of governance - The Republic upholds values like democracy, good governance, social justice, and integrity.
Article 11. Culture - The Constitution promotes the cultural heritage and rights of the people of Kenya.".to_string());

// Chapter Three
chunks_mut.push("CHAPTER THREE - CITIZENSHIP
Article 12. Entitlements of citizens - Citizens are entitled to the rights and freedoms guaranteed by the Constitution.
Article 13. Retention and acquisition of citizenship - Citizenship may be acquired, retained, or lost according to the Constitution and laws.
Article 14. Citizenship by birth - A person is a citizen by birth if born in Kenya or to Kenyan parents.
Article 15. Citizenship by registration - A person may acquire Kenyan citizenship by registration as provided by law.
Article 16. Dual citizenship - Citizens may hold dual nationality, provided it is permitted by law.
Article 17. Revocation of citizenship - Citizenship may be revoked in cases where a person acquired it by fraud or if they voluntarily renounce it.
Article 18. Legislation on citizenship - The national legislation governs the acquisition, loss, and registration of citizenship.".to_string());

// Chapter Four - Part 1
chunks_mut.push("CHAPTER FOUR - THE BILL OF RIGHTS
PART 1 - GENERAL PROVISIONS TO THE BILL OF RIGHTS
Article 19. Rights and fundamental freedoms - This article recognizes that rights and freedoms are inherent and that the Bill of Rights applies to all persons in Kenya.
Article 20. Application of Bill of Rights - The Bill of Rights applies to both public and private actions and ensures the protection of the rights and freedoms of individuals.
Article 21. Implementation of rights and fundamental freedoms - The state is obligated to respect, protect, promote, and fulfill the rights and freedoms in the Bill of Rights.
Article 22. Enforcement of Bill of Rights - Every person has the right to enforce their rights, and the courts may offer remedies to address violations.
Article 23. Authority of courts to uphold and enforce the Bill of Rights - Courts have the power to uphold and enforce the rights and freedoms in the Bill of Rights.
Article 24. Limitation of rights and fundamental freedoms - Rights may be limited by law, but such limitations must be reasonable and justifiable in a democratic society.
Article 25. Fundamental Rights and freedoms that may not be limited - Certain rights, such as the right to life and freedom from torture, cannot be limited under any circumstances.".to_string());

        // Chapter Four - Part 2
chunks_mut.push("PART 2 - CIVIL AND POLITICAL RIGHTS
Article 26. Right to life - Guarantees the right to life for every individual, and no person shall be deprived of their life arbitrarily.
Article 27. Equality and freedom from discrimination - Every person is entitled to equal protection and benefit of the law, and the law must not discriminate based on gender, race, ethnicity, religion, etc.
Article 28. Dignity - Every person has inherent dignity and the right to have their dignity respected and protected.
Article 29. Freedom and security of the person - Protects individuals from arbitrary arrest, detention, and ensures their freedom to move and live freely.
Article 30. Slavery, servitude, and forced labour - No one shall be held in slavery or servitude, and forced labour is prohibited.
Article 31. Privacy - Guarantees the right to privacy, including protection from arbitrary searches, seizure of personal information, and intrusion.
Article 32. Freedom of conscience, religion, belief and opinion - Guarantees the right to freedom of thought, conscience, religion, belief, and opinion, with the right to manifest those beliefs.
Article 33. Freedom of expression - Every person has the right to freely express their opinion, including freedom of the media, subject to reasonable limitations.
Article 34. Freedom of the media - Media freedom is guaranteed, and the state cannot interfere with the media’s ability to provide information and express diverse viewpoints.
Article 35. Access to information - Every citizen has the right to access information held by the state or other bodies, except in limited circumstances where the public interest is affected.
Article 36. Freedom of association - The right to freely associate with others, including forming or joining associations, is guaranteed.
Article 37. Assembly, demonstration, picketing and petition - Citizens have the right to assemble, demonstrate, picket, and petition in a peaceful manner.
Article 38. Political rights - Protects the right to participate in political life, including the right to vote, run for office, and freely express political beliefs.
Article 39. Freedom of movement and residence - Every person has the right to move freely within the country and to reside in any part of it.
Article 40. Protection of right to property - Individuals have the right to own, use, and dispose of property, and the state cannot arbitrarily take it without compensation.
Article 41. Labour relations - Guarantees the right to fair conditions of work, fair treatment, and the protection of employees' rights.
Article 42. Environment - Every person has the right to a clean and healthy environment, and the government must take steps to protect it.
Article 43. Economic and social rights - Guarantees economic, social, and cultural rights, such as the right to health, education, housing, and adequate standards of living.
Article 44. Language and culture - Everyone has the right to use their language and enjoy their culture, subject to the constitution and law.
Article 45. Marriage - Defines marriage and guarantees the right to enter into marriage freely, based on mutual consent, and lays down the principles of family.
Article 46. Consumer rights - Consumers are protected from unfair practices, and they have the right to safe goods and services and the right to redress for any grievances.
Article 47. Fair administrative action - Every person is entitled to fair administrative action, which includes the right to be heard before decisions affecting them are made.
Article 48. Access to justice - Guarantees the right to access justice and ensures that justice is available to all, regardless of one's status or financial capability.".to_string());

// Chapter Five
chunks_mut.push("CHAPTER FIVE - LAND AND ENVIRONMENT
Article 62. Land ownership - Specifies that land in Kenya belongs to the people of Kenya, and it is held in trust by the government for the people.
Article 63. Land tenure - Defines the different categories of land tenure and emphasizes the need for sustainable land ownership and usage.
Article 64. Ownership of land by government - The government may hold land on behalf of the people and can regulate its use in the interest of public good.
Article 65. Restriction on alienation of land - The government has the power to regulate the transfer of land to ensure land remains under responsible ownership, particularly regarding public interest.
Article 66. Government's powers over land - The government has the power to take land for public purposes, subject to compensation, and can regulate land use.
Article 67. National Land Commission - Establishes the National Land Commission, which is responsible for managing land administration and ensuring effective use and allocation of land.
Article 68. Land reforms - The government must implement land reforms to address historical land injustices and to ensure fair distribution of land.
Article 69. Environmental rights - The government has the duty to protect the environment and ensure sustainable use of natural resources.
Article 70. Enforcement of environmental rights - Provides for the establishment of mechanisms to enforce environmental rights, including the right to clean and healthy environments.".to_string());

// Chapter Six
chunks_mut.push("CHAPTER SIX - LEADERSHIP AND INTEGRITY
Article 73. Leadership and integrity - Leadership in public offices is based on the principles of good governance, transparency, accountability, and service to the people.
Article 74. Public office holders - Persons holding public office must adhere to ethical conduct and discharge their duties with integrity.
Article 75. Conduct of State officers - State officers must behave in a way that upholds the dignity of their office and maintain high standards of professional conduct.
Article 76. Protection of integrity of public office - Public office holders should not use their office for personal benefit, and misuse of office is prohibited.
Article 77. Restrictions on employment of State officers - State officers are prohibited from engaging in certain private activities that may conflict with their public duties.
Article 78. State officers and other employment - State officers are required to inform their employer about holding public office and any financial interests that might create a conflict of interest.
Article 79. Anti-corruption measures - Measures must be in place to address and combat corruption in public offices.
Article 80. Public accountability - There is a requirement for accountability in the use of public resources, and public officers are subject to oversight by appropriate authorities.".to_string());

// Chapter Seven
chunks_mut.push("CHAPTER SEVEN - THE EXECUTIVE
Article 87. The Executive - The executive authority of the Republic is vested in the President, who is the Head of State, the Head of Government, and the Commander-in-Chief of the Defense Forces.
Article 88. The President - The President is the chief executive officer of Kenya, elected through popular vote by Kenyan citizens.
Article 89. The Deputy President - The Deputy President is the deputy to the President and acts as President in the President's absence.
Article 90. Cabinet - The Cabinet consists of the President, Deputy President, and other appointed ministers who assist in running the government.
Article 91. Other executive organs - The executive includes public officers, the police, and other entities performing government functions, all accountable to the President and Parliament.
Article 92. Functions of the Executive - The Executive is responsible for the implementation of laws, the management of public affairs, and executing government policies.".to_string());

// Chapter Eight
chunks_mut.push("CHAPTER EIGHT - THE JUDICIARY
Article 159. Establishment of the Judiciary - The Judiciary is an independent and impartial branch of government that ensures the rule of law and the protection of rights.
Article 160. Independence of the Judiciary - The Judiciary is independent and should not be influenced by other branches of government or external forces.
Article 161. Judicial authority - Judicial authority is vested in the courts and includes the power to interpret the Constitution and apply the law.
Article 162. High Court - The High Court is established as a superior court with the jurisdiction to hear and determine matters concerning the interpretation of the Constitution.
Article 163. Court of Appeal - The Court of Appeal hears appeals from lower courts and is the highest court of appeal in Kenya.
Article 164. Judicial service - The Judicial Service Commission is tasked with appointing judicial officers and overseeing the administration of the courts.
Article 165. Access to justice - The Judiciary must ensure that all persons have access to justice and their rights are protected through fair legal processes.".to_string());

// Chapter Nine
chunks_mut.push("CHAPTER NINE - THE LEGISLATURE
Article 93. Establishment of Parliament - Parliament consists of the National Assembly and the Senate, which together form the legislature of Kenya.
Article 94. Functions of Parliament - Parliament has the responsibility to make laws, approve government policy, and oversee the executive.
Article 95. National Assembly - The National Assembly is the legislative chamber responsible for lawmaking, approving budgets, and checking the executive's work.
Article 96. Senate - The Senate represents the counties and serves as a check on national legislation by ensuring county interests are represented.
Article 97. Parliamentary privileges - Members of Parliament are granted certain privileges in the course of their duties to ensure their independence and effectiveness.
Article 98. Salaries and allowances of members of Parliament - The salaries and allowances of members of Parliament are provided by law to ensure their independence and fairness.
Article 99. Parliamentary elections - Elections to the National Assembly and Senate are conducted periodically as outlined in the Constitution, with universal suffrage and equal representation.".to_string());

// Add more chapters or continue adding parts as necessary

    });
    
    ic_cdk::println!("Constitution preloaded successfully with {} chunks", 
        CONSTITUTION_CHUNKS.with(|c| c.borrow().len()));
}

pub fn pre_upgrade_handler() {
    CONSTITUTION_CHUNKS.with(|chunks| {
        let chunks_data = chunks.borrow();
        match ic_cdk::storage::stable_save((chunks_data.clone(),)) {
            Ok(_) => (),
            Err(e) => ic_cdk::println!("Failed to save to stable memory: {}", e),
        }
    });
}

pub fn post_upgrade_handler() {
    match ic_cdk::storage::stable_restore::<(Vec<String>,)>() {
        Ok((chunks_data,)) => {
            CONSTITUTION_CHUNKS.with(|chunks| {
                *chunks.borrow_mut() = chunks_data;
            });
        }
        Err(e) => {
            // Initialize with default data instead of trapping
            ic_cdk::println!("Failed to restore from stable memory: {}", e);
            init_constitution(); // Call initialization function
        }
    }
}

pub fn get_all_chunks() -> Vec<String> {
    CONSTITUTION_CHUNKS.with(|chunks| chunks.borrow().clone())
}