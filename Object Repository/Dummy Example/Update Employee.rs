<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Update an employee record</description>
   <name>Update Employee</name>
   <tag></tag>
   <elementGuidId>d689ec52-11e6-4bce-9907-a14a583815ee</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;name\&quot;:\&quot;${employee_name}\&quot;,\&quot;salary\&quot;:\&quot;${salary_amount}\&quot;,\&quot;age\&quot;:\&quot;${employee_age}\&quot;}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>http://dummy.restapiexample.com/api/v1/update/${employee_id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'Bappy Sarkar'</defaultValue>
      <description></description>
      <id>418abf94-42a4-4050-866e-66b2a65aef8e</id>
      <masked>false</masked>
      <name>employee_name</name>
   </variables>
   <variables>
      <defaultValue>99999999999</defaultValue>
      <description></description>
      <id>8348084e-1231-4998-8e57-5494b0e39792</id>
      <masked>false</masked>
      <name>salary_amount</name>
   </variables>
   <variables>
      <defaultValue>26</defaultValue>
      <description></description>
      <id>b117143a-dfbc-4f90-bfe5-6d2b84e78612</id>
      <masked>false</masked>
      <name>employee_age</name>
   </variables>
   <variables>
      <defaultValue>23</defaultValue>
      <description></description>
      <id>8d98eb62-9602-424d-9d4f-6c2396425e48</id>
      <masked>false</masked>
      <name>employee_id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))


//assertThat(response.getResponseText()).isEqualTo(&quot;Katalon Test Project&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
