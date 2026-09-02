/// accepts a natural language prompt and processes it using the built in engine
    /// 
    pub async fn natural_language_prompt(&self, app_session: web::Data<AppSession>, user_session: Session, req: web::Form<NLPromptFormData>) -> impl Responder {
        tracing::info!("-> /nlprompt Requested;  natural_language_prompt();  prompt: \"{}\"", req.prompt);
        let user_session_details: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info

        let mut results_sbuf = String::with_capacity(500); // Single heap allocation
        /*let prompt = req.prompt.clone();
        let userid = user_session_details.get_userid_as_i64();
        //let patient_id = req.patient_id.clone();

                
        let nle = NaturalLanguageEngine::new( &app_session.get_full_path_language_model_file(),
                                                                 &app_session.get_full_path_tokenizer_file()
        ).await;

        let nle2 = NaturalLanguageEngine::new( &app_session.get_full_path_language_model_file(),
                                                            &app_session.get_full_path_tokenizer_file()
        ).await;
        
        // load the command controller structure, to manage proper use of the Language Engine
        let mut cmd: CommandController = CommandController::new(&app_session.get_full_path_command_mapping_file(), nle );
        let cmd2: CommandController = CommandController::new(&app_session.get_full_path_command_mapping_file(), nle2 );

        let cur_session: UserSession = user_session.get(constants::USER_SESSION).unwrap().unwrap();
       // let pdao = PatientDAO::new( app_session.get_db_connection() ).await;
       // let referenced_patient = cmd.get_referenced_patient(pdao, userid, prompt.clone());

       // if cur_session.user_authorizations.has_permission(p_id){
       // }

        //let classifer_results: Vec< (String, f32)> = cmd.get_classifier_rankings( prompt.clone() ).await;
        let classifer_results: Vec< (String, f32)> = cmd.get_classifier_rankings_filtered_for_permissions( prompt.clone(), cur_session.user_authorizations ).await;

        //results_sbuf.push_str("<H1>natural language prompt</H1>\n");
        results_sbuf.push_str( &NLECommandFormatter::get_nle_options_content(classifer_results, prompt, cmd2) );*/
        
        HttpResponse::Ok().body( results_sbuf )
    }
